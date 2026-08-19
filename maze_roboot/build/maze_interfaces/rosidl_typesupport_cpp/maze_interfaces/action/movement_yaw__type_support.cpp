// generated from rosidl_typesupport_cpp/resource/idl__type_support.cpp.em
// with input from maze_interfaces:action/MovementYaw.idl
// generated code does not contain a copyright notice

#include "cstddef"
#include "rosidl_runtime_c/message_type_support_struct.h"
#include "maze_interfaces/action/detail/movement_yaw__functions.h"
#include "maze_interfaces/action/detail/movement_yaw__struct.hpp"
#include "rosidl_typesupport_cpp/identifier.hpp"
#include "rosidl_typesupport_cpp/message_type_support.hpp"
#include "rosidl_typesupport_c/type_support_map.h"
#include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
#include "rosidl_typesupport_cpp/visibility_control.h"
#include "rosidl_typesupport_interface/macros.h"

namespace maze_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _MovementYaw_Goal_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _MovementYaw_Goal_type_support_ids_t;

static const _MovementYaw_Goal_type_support_ids_t _MovementYaw_Goal_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _MovementYaw_Goal_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _MovementYaw_Goal_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _MovementYaw_Goal_type_support_symbol_names_t _MovementYaw_Goal_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, maze_interfaces, action, MovementYaw_Goal)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, maze_interfaces, action, MovementYaw_Goal)),
  }
};

typedef struct _MovementYaw_Goal_type_support_data_t
{
  void * data[2];
} _MovementYaw_Goal_type_support_data_t;

static _MovementYaw_Goal_type_support_data_t _MovementYaw_Goal_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _MovementYaw_Goal_message_typesupport_map = {
  2,
  "maze_interfaces",
  &_MovementYaw_Goal_message_typesupport_ids.typesupport_identifier[0],
  &_MovementYaw_Goal_message_typesupport_symbol_names.symbol_name[0],
  &_MovementYaw_Goal_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t MovementYaw_Goal_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_MovementYaw_Goal_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
  &maze_interfaces__action__MovementYaw_Goal__get_type_hash,
  &maze_interfaces__action__MovementYaw_Goal__get_type_description,
  &maze_interfaces__action__MovementYaw_Goal__get_type_description_sources,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace maze_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<maze_interfaces::action::MovementYaw_Goal>()
{
  return &::maze_interfaces::action::rosidl_typesupport_cpp::MovementYaw_Goal_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, maze_interfaces, action, MovementYaw_Goal)() {
  return get_message_type_support_handle<maze_interfaces::action::MovementYaw_Goal>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__functions.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace maze_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _MovementYaw_Result_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _MovementYaw_Result_type_support_ids_t;

static const _MovementYaw_Result_type_support_ids_t _MovementYaw_Result_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _MovementYaw_Result_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _MovementYaw_Result_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _MovementYaw_Result_type_support_symbol_names_t _MovementYaw_Result_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, maze_interfaces, action, MovementYaw_Result)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, maze_interfaces, action, MovementYaw_Result)),
  }
};

typedef struct _MovementYaw_Result_type_support_data_t
{
  void * data[2];
} _MovementYaw_Result_type_support_data_t;

static _MovementYaw_Result_type_support_data_t _MovementYaw_Result_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _MovementYaw_Result_message_typesupport_map = {
  2,
  "maze_interfaces",
  &_MovementYaw_Result_message_typesupport_ids.typesupport_identifier[0],
  &_MovementYaw_Result_message_typesupport_symbol_names.symbol_name[0],
  &_MovementYaw_Result_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t MovementYaw_Result_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_MovementYaw_Result_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
  &maze_interfaces__action__MovementYaw_Result__get_type_hash,
  &maze_interfaces__action__MovementYaw_Result__get_type_description,
  &maze_interfaces__action__MovementYaw_Result__get_type_description_sources,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace maze_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<maze_interfaces::action::MovementYaw_Result>()
{
  return &::maze_interfaces::action::rosidl_typesupport_cpp::MovementYaw_Result_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, maze_interfaces, action, MovementYaw_Result)() {
  return get_message_type_support_handle<maze_interfaces::action::MovementYaw_Result>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__functions.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace maze_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _MovementYaw_Feedback_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _MovementYaw_Feedback_type_support_ids_t;

static const _MovementYaw_Feedback_type_support_ids_t _MovementYaw_Feedback_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _MovementYaw_Feedback_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _MovementYaw_Feedback_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _MovementYaw_Feedback_type_support_symbol_names_t _MovementYaw_Feedback_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, maze_interfaces, action, MovementYaw_Feedback)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, maze_interfaces, action, MovementYaw_Feedback)),
  }
};

typedef struct _MovementYaw_Feedback_type_support_data_t
{
  void * data[2];
} _MovementYaw_Feedback_type_support_data_t;

static _MovementYaw_Feedback_type_support_data_t _MovementYaw_Feedback_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _MovementYaw_Feedback_message_typesupport_map = {
  2,
  "maze_interfaces",
  &_MovementYaw_Feedback_message_typesupport_ids.typesupport_identifier[0],
  &_MovementYaw_Feedback_message_typesupport_symbol_names.symbol_name[0],
  &_MovementYaw_Feedback_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t MovementYaw_Feedback_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_MovementYaw_Feedback_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
  &maze_interfaces__action__MovementYaw_Feedback__get_type_hash,
  &maze_interfaces__action__MovementYaw_Feedback__get_type_description,
  &maze_interfaces__action__MovementYaw_Feedback__get_type_description_sources,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace maze_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<maze_interfaces::action::MovementYaw_Feedback>()
{
  return &::maze_interfaces::action::rosidl_typesupport_cpp::MovementYaw_Feedback_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, maze_interfaces, action, MovementYaw_Feedback)() {
  return get_message_type_support_handle<maze_interfaces::action::MovementYaw_Feedback>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__functions.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace maze_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _MovementYaw_SendGoal_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _MovementYaw_SendGoal_Request_type_support_ids_t;

static const _MovementYaw_SendGoal_Request_type_support_ids_t _MovementYaw_SendGoal_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _MovementYaw_SendGoal_Request_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _MovementYaw_SendGoal_Request_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _MovementYaw_SendGoal_Request_type_support_symbol_names_t _MovementYaw_SendGoal_Request_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, maze_interfaces, action, MovementYaw_SendGoal_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, maze_interfaces, action, MovementYaw_SendGoal_Request)),
  }
};

typedef struct _MovementYaw_SendGoal_Request_type_support_data_t
{
  void * data[2];
} _MovementYaw_SendGoal_Request_type_support_data_t;

static _MovementYaw_SendGoal_Request_type_support_data_t _MovementYaw_SendGoal_Request_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _MovementYaw_SendGoal_Request_message_typesupport_map = {
  2,
  "maze_interfaces",
  &_MovementYaw_SendGoal_Request_message_typesupport_ids.typesupport_identifier[0],
  &_MovementYaw_SendGoal_Request_message_typesupport_symbol_names.symbol_name[0],
  &_MovementYaw_SendGoal_Request_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t MovementYaw_SendGoal_Request_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_MovementYaw_SendGoal_Request_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
  &maze_interfaces__action__MovementYaw_SendGoal_Request__get_type_hash,
  &maze_interfaces__action__MovementYaw_SendGoal_Request__get_type_description,
  &maze_interfaces__action__MovementYaw_SendGoal_Request__get_type_description_sources,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace maze_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<maze_interfaces::action::MovementYaw_SendGoal_Request>()
{
  return &::maze_interfaces::action::rosidl_typesupport_cpp::MovementYaw_SendGoal_Request_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, maze_interfaces, action, MovementYaw_SendGoal_Request)() {
  return get_message_type_support_handle<maze_interfaces::action::MovementYaw_SendGoal_Request>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__functions.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace maze_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _MovementYaw_SendGoal_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _MovementYaw_SendGoal_Response_type_support_ids_t;

static const _MovementYaw_SendGoal_Response_type_support_ids_t _MovementYaw_SendGoal_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _MovementYaw_SendGoal_Response_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _MovementYaw_SendGoal_Response_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _MovementYaw_SendGoal_Response_type_support_symbol_names_t _MovementYaw_SendGoal_Response_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, maze_interfaces, action, MovementYaw_SendGoal_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, maze_interfaces, action, MovementYaw_SendGoal_Response)),
  }
};

typedef struct _MovementYaw_SendGoal_Response_type_support_data_t
{
  void * data[2];
} _MovementYaw_SendGoal_Response_type_support_data_t;

static _MovementYaw_SendGoal_Response_type_support_data_t _MovementYaw_SendGoal_Response_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _MovementYaw_SendGoal_Response_message_typesupport_map = {
  2,
  "maze_interfaces",
  &_MovementYaw_SendGoal_Response_message_typesupport_ids.typesupport_identifier[0],
  &_MovementYaw_SendGoal_Response_message_typesupport_symbol_names.symbol_name[0],
  &_MovementYaw_SendGoal_Response_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t MovementYaw_SendGoal_Response_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_MovementYaw_SendGoal_Response_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
  &maze_interfaces__action__MovementYaw_SendGoal_Response__get_type_hash,
  &maze_interfaces__action__MovementYaw_SendGoal_Response__get_type_description,
  &maze_interfaces__action__MovementYaw_SendGoal_Response__get_type_description_sources,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace maze_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<maze_interfaces::action::MovementYaw_SendGoal_Response>()
{
  return &::maze_interfaces::action::rosidl_typesupport_cpp::MovementYaw_SendGoal_Response_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, maze_interfaces, action, MovementYaw_SendGoal_Response)() {
  return get_message_type_support_handle<maze_interfaces::action::MovementYaw_SendGoal_Response>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__functions.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace maze_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _MovementYaw_SendGoal_Event_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _MovementYaw_SendGoal_Event_type_support_ids_t;

static const _MovementYaw_SendGoal_Event_type_support_ids_t _MovementYaw_SendGoal_Event_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _MovementYaw_SendGoal_Event_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _MovementYaw_SendGoal_Event_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _MovementYaw_SendGoal_Event_type_support_symbol_names_t _MovementYaw_SendGoal_Event_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, maze_interfaces, action, MovementYaw_SendGoal_Event)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, maze_interfaces, action, MovementYaw_SendGoal_Event)),
  }
};

typedef struct _MovementYaw_SendGoal_Event_type_support_data_t
{
  void * data[2];
} _MovementYaw_SendGoal_Event_type_support_data_t;

static _MovementYaw_SendGoal_Event_type_support_data_t _MovementYaw_SendGoal_Event_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _MovementYaw_SendGoal_Event_message_typesupport_map = {
  2,
  "maze_interfaces",
  &_MovementYaw_SendGoal_Event_message_typesupport_ids.typesupport_identifier[0],
  &_MovementYaw_SendGoal_Event_message_typesupport_symbol_names.symbol_name[0],
  &_MovementYaw_SendGoal_Event_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t MovementYaw_SendGoal_Event_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_MovementYaw_SendGoal_Event_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
  &maze_interfaces__action__MovementYaw_SendGoal_Event__get_type_hash,
  &maze_interfaces__action__MovementYaw_SendGoal_Event__get_type_description,
  &maze_interfaces__action__MovementYaw_SendGoal_Event__get_type_description_sources,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace maze_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<maze_interfaces::action::MovementYaw_SendGoal_Event>()
{
  return &::maze_interfaces::action::rosidl_typesupport_cpp::MovementYaw_SendGoal_Event_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, maze_interfaces, action, MovementYaw_SendGoal_Event)() {
  return get_message_type_support_handle<maze_interfaces::action::MovementYaw_SendGoal_Event>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
#include "rosidl_runtime_c/service_type_support_struct.h"
#include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
#include "rosidl_typesupport_cpp/service_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace maze_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _MovementYaw_SendGoal_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _MovementYaw_SendGoal_type_support_ids_t;

static const _MovementYaw_SendGoal_type_support_ids_t _MovementYaw_SendGoal_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _MovementYaw_SendGoal_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _MovementYaw_SendGoal_type_support_symbol_names_t;
#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _MovementYaw_SendGoal_type_support_symbol_names_t _MovementYaw_SendGoal_service_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, maze_interfaces, action, MovementYaw_SendGoal)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, maze_interfaces, action, MovementYaw_SendGoal)),
  }
};

typedef struct _MovementYaw_SendGoal_type_support_data_t
{
  void * data[2];
} _MovementYaw_SendGoal_type_support_data_t;

static _MovementYaw_SendGoal_type_support_data_t _MovementYaw_SendGoal_service_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _MovementYaw_SendGoal_service_typesupport_map = {
  2,
  "maze_interfaces",
  &_MovementYaw_SendGoal_service_typesupport_ids.typesupport_identifier[0],
  &_MovementYaw_SendGoal_service_typesupport_symbol_names.symbol_name[0],
  &_MovementYaw_SendGoal_service_typesupport_data.data[0],
};

static const rosidl_service_type_support_t MovementYaw_SendGoal_service_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_MovementYaw_SendGoal_service_typesupport_map),
  ::rosidl_typesupport_cpp::get_service_typesupport_handle_function,
  ::rosidl_typesupport_cpp::get_message_type_support_handle<maze_interfaces::action::MovementYaw_SendGoal_Request>(),
  ::rosidl_typesupport_cpp::get_message_type_support_handle<maze_interfaces::action::MovementYaw_SendGoal_Response>(),
  ::rosidl_typesupport_cpp::get_message_type_support_handle<maze_interfaces::action::MovementYaw_SendGoal_Event>(),
  &::rosidl_typesupport_cpp::service_create_event_message<maze_interfaces::action::MovementYaw_SendGoal>,
  &::rosidl_typesupport_cpp::service_destroy_event_message<maze_interfaces::action::MovementYaw_SendGoal>,
  &maze_interfaces__action__MovementYaw_SendGoal__get_type_hash,
  &maze_interfaces__action__MovementYaw_SendGoal__get_type_description,
  &maze_interfaces__action__MovementYaw_SendGoal__get_type_description_sources,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace maze_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
get_service_type_support_handle<maze_interfaces::action::MovementYaw_SendGoal>()
{
  return &::maze_interfaces::action::rosidl_typesupport_cpp::MovementYaw_SendGoal_service_type_support_handle;
}

}  // namespace rosidl_typesupport_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_cpp, maze_interfaces, action, MovementYaw_SendGoal)() {
  return ::rosidl_typesupport_cpp::get_service_type_support_handle<maze_interfaces::action::MovementYaw_SendGoal>();
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__functions.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace maze_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _MovementYaw_GetResult_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _MovementYaw_GetResult_Request_type_support_ids_t;

static const _MovementYaw_GetResult_Request_type_support_ids_t _MovementYaw_GetResult_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _MovementYaw_GetResult_Request_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _MovementYaw_GetResult_Request_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _MovementYaw_GetResult_Request_type_support_symbol_names_t _MovementYaw_GetResult_Request_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, maze_interfaces, action, MovementYaw_GetResult_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, maze_interfaces, action, MovementYaw_GetResult_Request)),
  }
};

typedef struct _MovementYaw_GetResult_Request_type_support_data_t
{
  void * data[2];
} _MovementYaw_GetResult_Request_type_support_data_t;

static _MovementYaw_GetResult_Request_type_support_data_t _MovementYaw_GetResult_Request_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _MovementYaw_GetResult_Request_message_typesupport_map = {
  2,
  "maze_interfaces",
  &_MovementYaw_GetResult_Request_message_typesupport_ids.typesupport_identifier[0],
  &_MovementYaw_GetResult_Request_message_typesupport_symbol_names.symbol_name[0],
  &_MovementYaw_GetResult_Request_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t MovementYaw_GetResult_Request_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_MovementYaw_GetResult_Request_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
  &maze_interfaces__action__MovementYaw_GetResult_Request__get_type_hash,
  &maze_interfaces__action__MovementYaw_GetResult_Request__get_type_description,
  &maze_interfaces__action__MovementYaw_GetResult_Request__get_type_description_sources,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace maze_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<maze_interfaces::action::MovementYaw_GetResult_Request>()
{
  return &::maze_interfaces::action::rosidl_typesupport_cpp::MovementYaw_GetResult_Request_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, maze_interfaces, action, MovementYaw_GetResult_Request)() {
  return get_message_type_support_handle<maze_interfaces::action::MovementYaw_GetResult_Request>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__functions.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace maze_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _MovementYaw_GetResult_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _MovementYaw_GetResult_Response_type_support_ids_t;

static const _MovementYaw_GetResult_Response_type_support_ids_t _MovementYaw_GetResult_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _MovementYaw_GetResult_Response_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _MovementYaw_GetResult_Response_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _MovementYaw_GetResult_Response_type_support_symbol_names_t _MovementYaw_GetResult_Response_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, maze_interfaces, action, MovementYaw_GetResult_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, maze_interfaces, action, MovementYaw_GetResult_Response)),
  }
};

typedef struct _MovementYaw_GetResult_Response_type_support_data_t
{
  void * data[2];
} _MovementYaw_GetResult_Response_type_support_data_t;

static _MovementYaw_GetResult_Response_type_support_data_t _MovementYaw_GetResult_Response_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _MovementYaw_GetResult_Response_message_typesupport_map = {
  2,
  "maze_interfaces",
  &_MovementYaw_GetResult_Response_message_typesupport_ids.typesupport_identifier[0],
  &_MovementYaw_GetResult_Response_message_typesupport_symbol_names.symbol_name[0],
  &_MovementYaw_GetResult_Response_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t MovementYaw_GetResult_Response_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_MovementYaw_GetResult_Response_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
  &maze_interfaces__action__MovementYaw_GetResult_Response__get_type_hash,
  &maze_interfaces__action__MovementYaw_GetResult_Response__get_type_description,
  &maze_interfaces__action__MovementYaw_GetResult_Response__get_type_description_sources,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace maze_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<maze_interfaces::action::MovementYaw_GetResult_Response>()
{
  return &::maze_interfaces::action::rosidl_typesupport_cpp::MovementYaw_GetResult_Response_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, maze_interfaces, action, MovementYaw_GetResult_Response)() {
  return get_message_type_support_handle<maze_interfaces::action::MovementYaw_GetResult_Response>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__functions.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace maze_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _MovementYaw_GetResult_Event_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _MovementYaw_GetResult_Event_type_support_ids_t;

static const _MovementYaw_GetResult_Event_type_support_ids_t _MovementYaw_GetResult_Event_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _MovementYaw_GetResult_Event_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _MovementYaw_GetResult_Event_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _MovementYaw_GetResult_Event_type_support_symbol_names_t _MovementYaw_GetResult_Event_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, maze_interfaces, action, MovementYaw_GetResult_Event)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, maze_interfaces, action, MovementYaw_GetResult_Event)),
  }
};

typedef struct _MovementYaw_GetResult_Event_type_support_data_t
{
  void * data[2];
} _MovementYaw_GetResult_Event_type_support_data_t;

static _MovementYaw_GetResult_Event_type_support_data_t _MovementYaw_GetResult_Event_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _MovementYaw_GetResult_Event_message_typesupport_map = {
  2,
  "maze_interfaces",
  &_MovementYaw_GetResult_Event_message_typesupport_ids.typesupport_identifier[0],
  &_MovementYaw_GetResult_Event_message_typesupport_symbol_names.symbol_name[0],
  &_MovementYaw_GetResult_Event_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t MovementYaw_GetResult_Event_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_MovementYaw_GetResult_Event_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
  &maze_interfaces__action__MovementYaw_GetResult_Event__get_type_hash,
  &maze_interfaces__action__MovementYaw_GetResult_Event__get_type_description,
  &maze_interfaces__action__MovementYaw_GetResult_Event__get_type_description_sources,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace maze_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<maze_interfaces::action::MovementYaw_GetResult_Event>()
{
  return &::maze_interfaces::action::rosidl_typesupport_cpp::MovementYaw_GetResult_Event_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, maze_interfaces, action, MovementYaw_GetResult_Event)() {
  return get_message_type_support_handle<maze_interfaces::action::MovementYaw_GetResult_Event>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/service_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace maze_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _MovementYaw_GetResult_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _MovementYaw_GetResult_type_support_ids_t;

static const _MovementYaw_GetResult_type_support_ids_t _MovementYaw_GetResult_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _MovementYaw_GetResult_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _MovementYaw_GetResult_type_support_symbol_names_t;
#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _MovementYaw_GetResult_type_support_symbol_names_t _MovementYaw_GetResult_service_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, maze_interfaces, action, MovementYaw_GetResult)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, maze_interfaces, action, MovementYaw_GetResult)),
  }
};

typedef struct _MovementYaw_GetResult_type_support_data_t
{
  void * data[2];
} _MovementYaw_GetResult_type_support_data_t;

static _MovementYaw_GetResult_type_support_data_t _MovementYaw_GetResult_service_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _MovementYaw_GetResult_service_typesupport_map = {
  2,
  "maze_interfaces",
  &_MovementYaw_GetResult_service_typesupport_ids.typesupport_identifier[0],
  &_MovementYaw_GetResult_service_typesupport_symbol_names.symbol_name[0],
  &_MovementYaw_GetResult_service_typesupport_data.data[0],
};

static const rosidl_service_type_support_t MovementYaw_GetResult_service_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_MovementYaw_GetResult_service_typesupport_map),
  ::rosidl_typesupport_cpp::get_service_typesupport_handle_function,
  ::rosidl_typesupport_cpp::get_message_type_support_handle<maze_interfaces::action::MovementYaw_GetResult_Request>(),
  ::rosidl_typesupport_cpp::get_message_type_support_handle<maze_interfaces::action::MovementYaw_GetResult_Response>(),
  ::rosidl_typesupport_cpp::get_message_type_support_handle<maze_interfaces::action::MovementYaw_GetResult_Event>(),
  &::rosidl_typesupport_cpp::service_create_event_message<maze_interfaces::action::MovementYaw_GetResult>,
  &::rosidl_typesupport_cpp::service_destroy_event_message<maze_interfaces::action::MovementYaw_GetResult>,
  &maze_interfaces__action__MovementYaw_GetResult__get_type_hash,
  &maze_interfaces__action__MovementYaw_GetResult__get_type_description,
  &maze_interfaces__action__MovementYaw_GetResult__get_type_description_sources,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace maze_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
get_service_type_support_handle<maze_interfaces::action::MovementYaw_GetResult>()
{
  return &::maze_interfaces::action::rosidl_typesupport_cpp::MovementYaw_GetResult_service_type_support_handle;
}

}  // namespace rosidl_typesupport_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_cpp, maze_interfaces, action, MovementYaw_GetResult)() {
  return ::rosidl_typesupport_cpp::get_service_type_support_handle<maze_interfaces::action::MovementYaw_GetResult>();
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__functions.h"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace maze_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _MovementYaw_FeedbackMessage_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _MovementYaw_FeedbackMessage_type_support_ids_t;

static const _MovementYaw_FeedbackMessage_type_support_ids_t _MovementYaw_FeedbackMessage_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _MovementYaw_FeedbackMessage_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _MovementYaw_FeedbackMessage_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _MovementYaw_FeedbackMessage_type_support_symbol_names_t _MovementYaw_FeedbackMessage_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, maze_interfaces, action, MovementYaw_FeedbackMessage)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, maze_interfaces, action, MovementYaw_FeedbackMessage)),
  }
};

typedef struct _MovementYaw_FeedbackMessage_type_support_data_t
{
  void * data[2];
} _MovementYaw_FeedbackMessage_type_support_data_t;

static _MovementYaw_FeedbackMessage_type_support_data_t _MovementYaw_FeedbackMessage_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _MovementYaw_FeedbackMessage_message_typesupport_map = {
  2,
  "maze_interfaces",
  &_MovementYaw_FeedbackMessage_message_typesupport_ids.typesupport_identifier[0],
  &_MovementYaw_FeedbackMessage_message_typesupport_symbol_names.symbol_name[0],
  &_MovementYaw_FeedbackMessage_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t MovementYaw_FeedbackMessage_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_MovementYaw_FeedbackMessage_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
  &maze_interfaces__action__MovementYaw_FeedbackMessage__get_type_hash,
  &maze_interfaces__action__MovementYaw_FeedbackMessage__get_type_description,
  &maze_interfaces__action__MovementYaw_FeedbackMessage__get_type_description_sources,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace maze_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<maze_interfaces::action::MovementYaw_FeedbackMessage>()
{
  return &::maze_interfaces::action::rosidl_typesupport_cpp::MovementYaw_FeedbackMessage_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, maze_interfaces, action, MovementYaw_FeedbackMessage)() {
  return get_message_type_support_handle<maze_interfaces::action::MovementYaw_FeedbackMessage>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

#include "action_msgs/msg/goal_status_array.hpp"
#include "action_msgs/srv/cancel_goal.hpp"
// already included above
// #include "maze_interfaces/action/detail/movement_yaw__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
#include "rosidl_runtime_c/action_type_support_struct.h"
#include "rosidl_typesupport_cpp/action_type_support.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_cpp/service_type_support.hpp"

namespace maze_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

static rosidl_action_type_support_t MovementYaw_action_type_support_handle = {
  NULL, NULL, NULL, NULL, NULL,
  &maze_interfaces__action__MovementYaw__get_type_hash,
  &maze_interfaces__action__MovementYaw__get_type_description,
  &maze_interfaces__action__MovementYaw__get_type_description_sources,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace maze_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_action_type_support_t *
get_action_type_support_handle<maze_interfaces::action::MovementYaw>()
{
  using ::maze_interfaces::action::rosidl_typesupport_cpp::MovementYaw_action_type_support_handle;
  // Thread-safe by always writing the same values to the static struct
  MovementYaw_action_type_support_handle.goal_service_type_support = get_service_type_support_handle<::maze_interfaces::action::MovementYaw::Impl::SendGoalService>();
  MovementYaw_action_type_support_handle.result_service_type_support = get_service_type_support_handle<::maze_interfaces::action::MovementYaw::Impl::GetResultService>();
  MovementYaw_action_type_support_handle.cancel_service_type_support = get_service_type_support_handle<::maze_interfaces::action::MovementYaw::Impl::CancelGoalService>();
  MovementYaw_action_type_support_handle.feedback_message_type_support = get_message_type_support_handle<::maze_interfaces::action::MovementYaw::Impl::FeedbackMessage>();
  MovementYaw_action_type_support_handle.status_message_type_support = get_message_type_support_handle<::maze_interfaces::action::MovementYaw::Impl::GoalStatusMessage>();
  return &MovementYaw_action_type_support_handle;
}

}  // namespace rosidl_typesupport_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_action_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__ACTION_SYMBOL_NAME(rosidl_typesupport_cpp, maze_interfaces, action, MovementYaw)() {
  return ::rosidl_typesupport_cpp::get_action_type_support_handle<maze_interfaces::action::MovementYaw>();
}

#ifdef __cplusplus
}
#endif
