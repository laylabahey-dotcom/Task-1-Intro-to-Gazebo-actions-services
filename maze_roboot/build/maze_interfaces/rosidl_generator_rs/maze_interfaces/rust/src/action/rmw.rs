
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_Goal() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementX_Goal__init(msg: *mut MovementX_Goal) -> bool;
    fn maze_interfaces__action__MovementX_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementX_Goal>, size: usize) -> bool;
    fn maze_interfaces__action__MovementX_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementX_Goal>);
    fn maze_interfaces__action__MovementX_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementX_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementX_Goal>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementX_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementX_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub target_x: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub linear_speed: f32,

}



impl Default for MovementX_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementX_Goal__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementX_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementX_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementX_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementX_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementX_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_Goal() }
  }
}


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_Result() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementX_Result__init(msg: *mut MovementX_Result) -> bool;
    fn maze_interfaces__action__MovementX_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementX_Result>, size: usize) -> bool;
    fn maze_interfaces__action__MovementX_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementX_Result>);
    fn maze_interfaces__action__MovementX_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementX_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementX_Result>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementX_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementX_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub final_distance_traveled: f32,

}



impl Default for MovementX_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementX_Result__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementX_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementX_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementX_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementX_Result where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementX_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_Result() }
  }
}


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementX_Feedback__init(msg: *mut MovementX_Feedback) -> bool;
    fn maze_interfaces__action__MovementX_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementX_Feedback>, size: usize) -> bool;
    fn maze_interfaces__action__MovementX_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementX_Feedback>);
    fn maze_interfaces__action__MovementX_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementX_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementX_Feedback>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementX_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementX_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub distance_traveled: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub distance_remaining: f32,

}



impl Default for MovementX_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementX_Feedback__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementX_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementX_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementX_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementX_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementX_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_Feedback() }
  }
}


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementX_FeedbackMessage__init(msg: *mut MovementX_FeedbackMessage) -> bool;
    fn maze_interfaces__action__MovementX_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementX_FeedbackMessage>, size: usize) -> bool;
    fn maze_interfaces__action__MovementX_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementX_FeedbackMessage>);
    fn maze_interfaces__action__MovementX_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementX_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementX_FeedbackMessage>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementX_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementX_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MovementX_Feedback,

}



impl Default for MovementX_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementX_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementX_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementX_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementX_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementX_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementX_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_FeedbackMessage() }
  }
}


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_Goal() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementYaw_Goal__init(msg: *mut MovementYaw_Goal) -> bool;
    fn maze_interfaces__action__MovementYaw_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_Goal>, size: usize) -> bool;
    fn maze_interfaces__action__MovementYaw_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_Goal>);
    fn maze_interfaces__action__MovementYaw_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementYaw_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_Goal>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementYaw_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementYaw_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub target_yaw: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub angular_speed: f32,

}



impl Default for MovementYaw_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementYaw_Goal__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementYaw_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementYaw_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementYaw_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementYaw_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementYaw_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_Goal() }
  }
}


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_Result() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementYaw_Result__init(msg: *mut MovementYaw_Result) -> bool;
    fn maze_interfaces__action__MovementYaw_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_Result>, size: usize) -> bool;
    fn maze_interfaces__action__MovementYaw_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_Result>);
    fn maze_interfaces__action__MovementYaw_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementYaw_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_Result>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementYaw_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementYaw_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub final_yaw_traveled: f32,

}



impl Default for MovementYaw_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementYaw_Result__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementYaw_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementYaw_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementYaw_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementYaw_Result where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementYaw_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_Result() }
  }
}


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementYaw_Feedback__init(msg: *mut MovementYaw_Feedback) -> bool;
    fn maze_interfaces__action__MovementYaw_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_Feedback>, size: usize) -> bool;
    fn maze_interfaces__action__MovementYaw_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_Feedback>);
    fn maze_interfaces__action__MovementYaw_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementYaw_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_Feedback>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementYaw_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementYaw_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_traveled: f32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw_remaining: f32,

}



impl Default for MovementYaw_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementYaw_Feedback__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementYaw_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementYaw_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementYaw_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementYaw_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementYaw_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_Feedback() }
  }
}


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementYaw_FeedbackMessage__init(msg: *mut MovementYaw_FeedbackMessage) -> bool;
    fn maze_interfaces__action__MovementYaw_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_FeedbackMessage>, size: usize) -> bool;
    fn maze_interfaces__action__MovementYaw_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_FeedbackMessage>);
    fn maze_interfaces__action__MovementYaw_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementYaw_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_FeedbackMessage>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementYaw_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementYaw_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MovementYaw_Feedback,

}



impl Default for MovementYaw_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementYaw_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementYaw_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementYaw_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementYaw_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementYaw_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementYaw_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_FeedbackMessage() }
  }
}




#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementX_SendGoal_Request__init(msg: *mut MovementX_SendGoal_Request) -> bool;
    fn maze_interfaces__action__MovementX_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementX_SendGoal_Request>, size: usize) -> bool;
    fn maze_interfaces__action__MovementX_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementX_SendGoal_Request>);
    fn maze_interfaces__action__MovementX_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementX_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementX_SendGoal_Request>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementX_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementX_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MovementX_Goal,

}



impl Default for MovementX_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementX_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementX_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementX_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementX_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementX_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementX_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_SendGoal_Request() }
  }
}


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementX_SendGoal_Response__init(msg: *mut MovementX_SendGoal_Response) -> bool;
    fn maze_interfaces__action__MovementX_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementX_SendGoal_Response>, size: usize) -> bool;
    fn maze_interfaces__action__MovementX_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementX_SendGoal_Response>);
    fn maze_interfaces__action__MovementX_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementX_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementX_SendGoal_Response>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementX_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementX_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MovementX_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementX_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementX_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementX_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementX_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementX_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementX_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_SendGoal_Response() }
  }
}


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementX_GetResult_Request__init(msg: *mut MovementX_GetResult_Request) -> bool;
    fn maze_interfaces__action__MovementX_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementX_GetResult_Request>, size: usize) -> bool;
    fn maze_interfaces__action__MovementX_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementX_GetResult_Request>);
    fn maze_interfaces__action__MovementX_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementX_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementX_GetResult_Request>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementX_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementX_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MovementX_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementX_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementX_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementX_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementX_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementX_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementX_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_GetResult_Request() }
  }
}


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementX_GetResult_Response__init(msg: *mut MovementX_GetResult_Response) -> bool;
    fn maze_interfaces__action__MovementX_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementX_GetResult_Response>, size: usize) -> bool;
    fn maze_interfaces__action__MovementX_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementX_GetResult_Response>);
    fn maze_interfaces__action__MovementX_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementX_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementX_GetResult_Response>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementX_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementX_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MovementX_Result,

}



impl Default for MovementX_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementX_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementX_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementX_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementX_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementX_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementX_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementX_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementX_GetResult_Response() }
  }
}


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementYaw_SendGoal_Request__init(msg: *mut MovementYaw_SendGoal_Request) -> bool;
    fn maze_interfaces__action__MovementYaw_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_SendGoal_Request>, size: usize) -> bool;
    fn maze_interfaces__action__MovementYaw_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_SendGoal_Request>);
    fn maze_interfaces__action__MovementYaw_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementYaw_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_SendGoal_Request>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementYaw_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementYaw_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MovementYaw_Goal,

}



impl Default for MovementYaw_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementYaw_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementYaw_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementYaw_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementYaw_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementYaw_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementYaw_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_SendGoal_Request() }
  }
}


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementYaw_SendGoal_Response__init(msg: *mut MovementYaw_SendGoal_Response) -> bool;
    fn maze_interfaces__action__MovementYaw_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_SendGoal_Response>, size: usize) -> bool;
    fn maze_interfaces__action__MovementYaw_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_SendGoal_Response>);
    fn maze_interfaces__action__MovementYaw_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementYaw_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_SendGoal_Response>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementYaw_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementYaw_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MovementYaw_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementYaw_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementYaw_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementYaw_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementYaw_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementYaw_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementYaw_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_SendGoal_Response() }
  }
}


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementYaw_GetResult_Request__init(msg: *mut MovementYaw_GetResult_Request) -> bool;
    fn maze_interfaces__action__MovementYaw_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_GetResult_Request>, size: usize) -> bool;
    fn maze_interfaces__action__MovementYaw_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_GetResult_Request>);
    fn maze_interfaces__action__MovementYaw_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementYaw_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_GetResult_Request>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementYaw_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementYaw_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MovementYaw_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementYaw_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementYaw_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementYaw_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementYaw_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementYaw_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementYaw_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_GetResult_Request() }
  }
}


#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "maze_interfaces__rosidl_generator_c")]
extern "C" {
    fn maze_interfaces__action__MovementYaw_GetResult_Response__init(msg: *mut MovementYaw_GetResult_Response) -> bool;
    fn maze_interfaces__action__MovementYaw_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_GetResult_Response>, size: usize) -> bool;
    fn maze_interfaces__action__MovementYaw_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_GetResult_Response>);
    fn maze_interfaces__action__MovementYaw_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MovementYaw_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MovementYaw_GetResult_Response>) -> bool;
}

// Corresponds to maze_interfaces__action__MovementYaw_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MovementYaw_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MovementYaw_Result,

}



impl Default for MovementYaw_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !maze_interfaces__action__MovementYaw_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to maze_interfaces__action__MovementYaw_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MovementYaw_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { maze_interfaces__action__MovementYaw_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MovementYaw_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MovementYaw_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "maze_interfaces/action/MovementYaw_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__maze_interfaces__action__MovementYaw_GetResult_Response() }
  }
}






#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__maze_interfaces__action__MovementX_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to maze_interfaces__action__MovementX_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MovementX_SendGoal;

impl rosidl_runtime_rs::Service for MovementX_SendGoal {
    type Request = MovementX_SendGoal_Request;
    type Response = MovementX_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__maze_interfaces__action__MovementX_SendGoal() }
    }
}




#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__maze_interfaces__action__MovementX_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to maze_interfaces__action__MovementX_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MovementX_GetResult;

impl rosidl_runtime_rs::Service for MovementX_GetResult {
    type Request = MovementX_GetResult_Request;
    type Response = MovementX_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__maze_interfaces__action__MovementX_GetResult() }
    }
}




#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__maze_interfaces__action__MovementYaw_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to maze_interfaces__action__MovementYaw_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MovementYaw_SendGoal;

impl rosidl_runtime_rs::Service for MovementYaw_SendGoal {
    type Request = MovementYaw_SendGoal_Request;
    type Response = MovementYaw_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__maze_interfaces__action__MovementYaw_SendGoal() }
    }
}




#[link(name = "maze_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__maze_interfaces__action__MovementYaw_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to maze_interfaces__action__MovementYaw_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MovementYaw_GetResult;

impl rosidl_runtime_rs::Service for MovementYaw_GetResult {
    type Request = MovementYaw_GetResult_Request;
    type Response = MovementYaw_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__maze_interfaces__action__MovementYaw_GetResult() }
    }
}


