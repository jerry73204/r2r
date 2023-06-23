pub mod msg { use super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct PrimaryControl { pub header : std_msgs :: msg :: Header , pub active : bool , pub estop : bool , pub steering_command : f32 , pub throttle_command : f32 , pub brake_command : f32 } impl WrappedTypesupport for PrimaryControl { type CStruct = marti_dbw_msgs__msg__PrimaryControl ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__marti_dbw_msgs__msg__PrimaryControl () } } fn create_msg () -> * mut marti_dbw_msgs__msg__PrimaryControl { unsafe { marti_dbw_msgs__msg__PrimaryControl__create () } } fn destroy_msg (msg : * mut marti_dbw_msgs__msg__PrimaryControl) -> () { unsafe { marti_dbw_msgs__msg__PrimaryControl__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> PrimaryControl { PrimaryControl { header : std_msgs :: msg :: Header :: from_native (& msg . header) , active : msg . active , estop : msg . estop , steering_command : msg . steering_command , throttle_command : msg . throttle_command , brake_command : msg . brake_command , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { self . header . copy_to_native (& mut msg . header) ; msg . active = self . active ; msg . estop = self . estop ; msg . steering_command = self . steering_command ; msg . throttle_command = self . throttle_command ; msg . brake_command = self . brake_command ; } } impl Default for PrimaryControl { fn default () -> Self { let msg_native = WrappedNativeMsg :: < PrimaryControl > :: new () ; PrimaryControl :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct PrimaryFeedback { pub header : std_msgs :: msg :: Header , pub present : bool , pub robotic_mode : bool , pub steering_command : f32 , pub steering_measure : f32 , pub throttle_command : f32 , pub throttle_measure : f32 , pub brake_command : f32 , pub brake_measure : f32 , pub estop_command : bool , pub estop_measure : bool , pub error_steering : bool , pub error_throttle : bool , pub error_brake : bool , pub error_other : bool } impl WrappedTypesupport for PrimaryFeedback { type CStruct = marti_dbw_msgs__msg__PrimaryFeedback ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__marti_dbw_msgs__msg__PrimaryFeedback () } } fn create_msg () -> * mut marti_dbw_msgs__msg__PrimaryFeedback { unsafe { marti_dbw_msgs__msg__PrimaryFeedback__create () } } fn destroy_msg (msg : * mut marti_dbw_msgs__msg__PrimaryFeedback) -> () { unsafe { marti_dbw_msgs__msg__PrimaryFeedback__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> PrimaryFeedback { PrimaryFeedback { header : std_msgs :: msg :: Header :: from_native (& msg . header) , present : msg . present , robotic_mode : msg . robotic_mode , steering_command : msg . steering_command , steering_measure : msg . steering_measure , throttle_command : msg . throttle_command , throttle_measure : msg . throttle_measure , brake_command : msg . brake_command , brake_measure : msg . brake_measure , estop_command : msg . estop_command , estop_measure : msg . estop_measure , error_steering : msg . error_steering , error_throttle : msg . error_throttle , error_brake : msg . error_brake , error_other : msg . error_other , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { self . header . copy_to_native (& mut msg . header) ; msg . present = self . present ; msg . robotic_mode = self . robotic_mode ; msg . steering_command = self . steering_command ; msg . steering_measure = self . steering_measure ; msg . throttle_command = self . throttle_command ; msg . throttle_measure = self . throttle_measure ; msg . brake_command = self . brake_command ; msg . brake_measure = self . brake_measure ; msg . estop_command = self . estop_command ; msg . estop_measure = self . estop_measure ; msg . error_steering = self . error_steering ; msg . error_throttle = self . error_throttle ; msg . error_brake = self . error_brake ; msg . error_other = self . error_other ; } } impl Default for PrimaryFeedback { fn default () -> Self { let msg_native = WrappedNativeMsg :: < PrimaryFeedback > :: new () ; PrimaryFeedback :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct TransmissionFeedback { pub header : std_msgs :: msg :: Header , pub current_range : std :: string :: String , pub stable : bool , pub reverse : bool , pub forward : bool } impl WrappedTypesupport for TransmissionFeedback { type CStruct = marti_dbw_msgs__msg__TransmissionFeedback ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__marti_dbw_msgs__msg__TransmissionFeedback () } } fn create_msg () -> * mut marti_dbw_msgs__msg__TransmissionFeedback { unsafe { marti_dbw_msgs__msg__TransmissionFeedback__create () } } fn destroy_msg (msg : * mut marti_dbw_msgs__msg__TransmissionFeedback) -> () { unsafe { marti_dbw_msgs__msg__TransmissionFeedback__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> TransmissionFeedback { TransmissionFeedback { header : std_msgs :: msg :: Header :: from_native (& msg . header) , current_range : msg . current_range . to_str () . to_owned () , stable : msg . stable , reverse : msg . reverse , forward : msg . forward , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { self . header . copy_to_native (& mut msg . header) ; msg . current_range . assign (& self . current_range) ; msg . stable = self . stable ; msg . reverse = self . reverse ; msg . forward = self . forward ; } } impl Default for TransmissionFeedback { fn default () -> Self { let msg_native = WrappedNativeMsg :: < TransmissionFeedback > :: new () ; TransmissionFeedback :: from_native (& msg_native) } } }