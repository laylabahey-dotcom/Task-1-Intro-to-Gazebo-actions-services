import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/julian/Task-1-Intro-to-Gazebo-actions-services/maze_roboot/install/robot_maze'
