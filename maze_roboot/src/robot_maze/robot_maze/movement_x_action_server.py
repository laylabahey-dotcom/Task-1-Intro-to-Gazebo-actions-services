import rclpy
from rclpy.action import ActionServer
from rclpy.node import Node
from nav_msgs.msg import Odometry #library used to for position calculations, required in subscriber
from geometry_msgs.msg import Twist #carries linear and angular data types, which functions better than Float32


class MoveX(Node):
    def __init__(self):
        super().__init__('movement_x_server')
        #self._action_server = ActionServer(self, Float32, 'movement_x', self.execute_callback)

        self.subscription = self.create_subscription(Odometry, 'position', self.position_callback, 10)

    def position_callback(self, msg):
            position = msg.pose.pose.position
            self.get_logger().info(position)

def main():
    rclpy.init()

    node = MoveX()

    rclpy.spin(node)

    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()
