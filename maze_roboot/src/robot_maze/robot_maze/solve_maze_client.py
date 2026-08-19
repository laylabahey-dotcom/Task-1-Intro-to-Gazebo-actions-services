import math
import time

import rclpy
from rclpy.action import ActionClient
from rclpy.node import Node

from std_srvs.srv import SetBool

from maze_solver.action import MoveRobotX, MoveRobotYaw

HALF_PI = math.pi / 2.0  # 90 degrees
WALL_SETTLE_S = 6.0 


class MazeClient(Node):

    def __init__(self):
        super().__init__('maze_client')

        self._move_x_client = ActionClient(self, MoveRobotX, 'move_robot_x')
        self._move_yaw_client = ActionClient(self, MoveRobotYaw, 'move_robot_yaw')
        self._wall_client = self.create_client(SetBool, '/toggle_walls_1_2')

        self.get_logger().info("maze_client ready.")

    def move_x(self, distance: float, server_wait_timeout: float = 10.0) -> bool:
        self.get_logger().info(f"move_x: requesting distance={distance:.3f} m")

        if not self._move_x_client.wait_for_server(timeout_sec=server_wait_timeout):
            self.get_logger().error("move_robot_x action server not available.")
            return False

        goal_msg = MoveRobotX.Goal()
        goal_msg.distance = distance

        send_future = self._move_x_client.send_goal_async(
            goal_msg, feedback_callback=self._move_x_feedback)
        rclpy.spin_until_future_complete(self, send_future)
        goal_handle = send_future.result()

        if not goal_handle.accepted:
            self.get_logger().error("move_robot_x goal was rejected.")
            return False

        result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, result_future)
        result = result_future.result().result

        self.get_logger().info(f"move_x result: {result.message}")
        return result.success

    def move_yaw(self, target_yaw: float, server_wait_timeout: float = 10.0) -> bool:
        self.get_logger().info(f"move_yaw: requesting target_yaw={target_yaw:.3f} rad")

        if not self._move_yaw_client.wait_for_server(timeout_sec=server_wait_timeout):
            self.get_logger().error("move_robot_yaw action server not available.")
            return False

        goal_msg = MoveRobotYaw.Goal()
        goal_msg.target_yaw = target_yaw

        send_future = self._move_yaw_client.send_goal_async(
            goal_msg, feedback_callback=self._move_yaw_feedback)
        rclpy.spin_until_future_complete(self, send_future)
        goal_handle = send_future.result()

        if not goal_handle.accepted:
            self.get_logger().error("move_robot_yaw goal was rejected.")
            return False

        result_future = goal_handle.get_result_async()
        rclpy.spin_until_future_complete(self, result_future)
        result = result_future.result().result

        self.get_logger().info(f"move_yaw result: {result.message}")
        return result.success

    def toggle_walls(self, data: bool, server_wait_timeout: float = 5.0) -> bool:
        if not self._wall_client.wait_for_service(timeout_sec=server_wait_timeout):
            self.get_logger().error("/toggle_walls_1_2 service not available.")
            return False

        req = SetBool.Request()
        req.data = data
        future = self._wall_client.call_async(req)
        rclpy.spin_until_future_complete(self, future)
        response = future.result()
        if response is None:
            self.get_logger().error("toggle_walls: no response from service.")
            return False

        self.get_logger().info(f"toggle_walls({data}): {response.message}")
        if response.success:
            self.get_logger().info(f"Waiting {WALL_SETTLE_S:.1f}s for walls to finish moving.")
            time.sleep(WALL_SETTLE_S)
        return response.success

    def _move_x_feedback(self, feedback_msg):
        fb = feedback_msg.feedback
        self.get_logger().debug(
            f"move_x feedback: travelled={fb.distance_travelled:.3f}, "
            f"remaining={fb.distance_remaining:.3f}"
        )

    def _move_yaw_feedback(self, feedback_msg):
        fb = feedback_msg.feedback
        self.get_logger().debug(
            f"move_yaw feedback: turned={fb.yaw_turned:.3f}, "
            f"remaining={fb.yaw_remaining:.3f}"
        )

    def solve_maze(self) -> bool:
        """
        Short path using the retractable red walls.

        Maze (metres), robot spawned at (0.5, 0.5) facing +X:
          y=0.5  bottom corridor (start)
          y=1.0  wall 1  — starts DOWN; True lifts it
          y=1.5  middle corridor
          y=2.0  wall 2  — starts UP; False lifts it (and drops wall 1)
          y=2.5  top corridor → finish at x≈5.0

        true  = wall 1 up, wall 2 down
        false = wall 1 down, wall 2 up
        """
        self.get_logger().info("solve_maze: starting from spawn (0.5, 0.5), yaw=0.")

        steps = [
            ("toggle_walls", True, "open wall 1 (start -> middle)"),
            ("move_yaw", HALF_PI, "face +Y (left 90 deg)"),
            ("move_x", 0.95, "through wall 1, stop short of wall 2"),
            ("toggle_walls", False, "open wall 2 (middle -> top)"),
            ("move_x", 1.15, "through wall 2 to top corridor"),
            ("move_yaw", -HALF_PI, "face +X (right 90 deg)"),
            ("move_x", 4.5, "drive east onto the finish line"),
        ]

        for i, (step_type, value, note) in enumerate(steps, start=1):
            self.get_logger().info(f"solve_maze step {i}/{len(steps)}: {step_type}({value}) — {note}")

            if step_type == "move_x":
                ok = self.move_x(value)
            elif step_type == "move_yaw":
                ok = self.move_yaw(value)
            elif step_type == "toggle_walls":
                ok = self.toggle_walls(value)
            else:
                ok = False

            if not ok:
                self.get_logger().error(
                    f"solve_maze: step {i} {step_type}({value}) failed — aborting."
                )
                return False

        self.get_logger().info("solve_maze: sequence complete — robot should be on the finish line.")
        return True


def main(args=None):
    rclpy.init(args=args)
    node = MazeClient()
    try:
        success = node.solve_maze()
        if success:
            node.get_logger().info("Maze solved successfully!")
        else:
            node.get_logger().error("Maze run did not complete successfully.")
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()
        if rclpy.ok():
            rclpy.shutdown()


if __name__ == '__main__':
    main()
