import time
from maze_interfaces.action import MovementX
import rclpy
from rclpy.action import ActionServer
from rclpy.node import Node
from nav_msgs.msg import Odometry #library used to for position calculations, required in subscriber
from geometry_msgs.msg import Twist #carries linear and angular data types, which functions better than Float32


class MoveX(Node):
    def __init__(self):
        super().__init__('movement_x_server')
        self._action_server = ActionServer(self, MovementX, 'movement_x', self.execute_callback)

        self.odom = None
        self.subscription = self.create_subscription(Odometry, '/odom', self.odom_callback, 10)
        self.cmd_vel_pub = self.create_publisher(Twist, '/cmd_vel', 10)
    def execute_callback(self, goal_handle):

        # get distance, speed, and starting position from the goal request

        target_distance = target_distance = abs(goal_handle.request.target_x)
        speed = goal_handle.request.linear_speed or 0.15
        start_x = self.odom.pose.pose.position.x
        start_y = self.odom.pose.pose.position.y

        while True:
            # calculate the distance moved based on the current position and the starting position
            dy = self.odom.pose.pose.position.y - start_y
            dx = self.odom.pose.pose.position.x - start_x
            distance_moved = (dx ** 2 + dy ** 2) ** 0.5

            if target_distance - distance_moved <= 0.02:
                break

            if target_distance < 0:
                speed = -speed

            twist = Twist()
            twist.linear.x = speed
            self.cmd_vel_pub.publish(twist)
            time.sleep(0.02)

        self.cmd_vel_pub.publish(Twist())
        goal_handle.succeed()
        return MovementX.Result()


        



    def odom_callback(self, msg):
        self.odom = msg

def main():
    rclpy.init()

    node = MoveX()

    rclpy.spin(node)

    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()
