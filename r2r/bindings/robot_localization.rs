pub mod srv { # [allow (non_snake_case)] pub mod FromLL { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__robot_localization__srv__FromLL () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { pub ll_point : geographic_msgs :: msg :: GeoPoint } impl WrappedTypesupport for Request { type CStruct = robot_localization__srv__FromLL_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__robot_localization__srv__FromLL_Request () } } fn create_msg () -> * mut robot_localization__srv__FromLL_Request { unsafe { robot_localization__srv__FromLL_Request__create () } } fn destroy_msg (msg : * mut robot_localization__srv__FromLL_Request) -> () { unsafe { robot_localization__srv__FromLL_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { ll_point : geographic_msgs :: msg :: GeoPoint :: from_native (& msg . ll_point) , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { self . ll_point . copy_to_native (& mut msg . ll_point) ; } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub map_point : geometry_msgs :: msg :: Point } impl WrappedTypesupport for Response { type CStruct = robot_localization__srv__FromLL_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__robot_localization__srv__FromLL_Response () } } fn create_msg () -> * mut robot_localization__srv__FromLL_Response { unsafe { robot_localization__srv__FromLL_Response__create () } } fn destroy_msg (msg : * mut robot_localization__srv__FromLL_Response) -> () { unsafe { robot_localization__srv__FromLL_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { map_point : geometry_msgs :: msg :: Point :: from_native (& msg . map_point) , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { self . map_point . copy_to_native (& mut msg . map_point) ; } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod GetState { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__robot_localization__srv__GetState () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { pub time_stamp : builtin_interfaces :: msg :: Time , pub frame_id : std :: string :: String } impl WrappedTypesupport for Request { type CStruct = robot_localization__srv__GetState_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__robot_localization__srv__GetState_Request () } } fn create_msg () -> * mut robot_localization__srv__GetState_Request { unsafe { robot_localization__srv__GetState_Request__create () } } fn destroy_msg (msg : * mut robot_localization__srv__GetState_Request) -> () { unsafe { robot_localization__srv__GetState_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { time_stamp : builtin_interfaces :: msg :: Time :: from_native (& msg . time_stamp) , frame_id : msg . frame_id . to_str () . to_owned () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { self . time_stamp . copy_to_native (& mut msg . time_stamp) ; msg . frame_id . assign (& self . frame_id) ; } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub state : Vec < f64 > , pub covariance : Vec < f64 > } impl WrappedTypesupport for Response { type CStruct = robot_localization__srv__GetState_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__robot_localization__srv__GetState_Response () } } fn create_msg () -> * mut robot_localization__srv__GetState_Response { unsafe { robot_localization__srv__GetState_Response__create () } } fn destroy_msg (msg : * mut robot_localization__srv__GetState_Response) -> () { unsafe { robot_localization__srv__GetState_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { state : msg . state . to_vec () , covariance : msg . covariance . to_vec () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { assert_eq ! (self . state . len () , 15usize , "Field {} is fixed size of {}!" , "state" , 15usize) ; msg . state . copy_from_slice (& self . state [.. 15usize]) ; assert_eq ! (self . covariance . len () , 225usize , "Field {} is fixed size of {}!" , "covariance" , 225usize) ; msg . covariance . copy_from_slice (& self . covariance [.. 225usize]) ; } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod SetDatum { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__robot_localization__srv__SetDatum () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { pub geo_pose : geographic_msgs :: msg :: GeoPose } impl WrappedTypesupport for Request { type CStruct = robot_localization__srv__SetDatum_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__robot_localization__srv__SetDatum_Request () } } fn create_msg () -> * mut robot_localization__srv__SetDatum_Request { unsafe { robot_localization__srv__SetDatum_Request__create () } } fn destroy_msg (msg : * mut robot_localization__srv__SetDatum_Request) -> () { unsafe { robot_localization__srv__SetDatum_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { geo_pose : geographic_msgs :: msg :: GeoPose :: from_native (& msg . geo_pose) , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { self . geo_pose . copy_to_native (& mut msg . geo_pose) ; } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { } impl WrappedTypesupport for Response { type CStruct = robot_localization__srv__SetDatum_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__robot_localization__srv__SetDatum_Response () } } fn create_msg () -> * mut robot_localization__srv__SetDatum_Response { unsafe { robot_localization__srv__SetDatum_Response__create () } } fn destroy_msg (msg : * mut robot_localization__srv__SetDatum_Response) -> () { unsafe { robot_localization__srv__SetDatum_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod SetPose { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__robot_localization__srv__SetPose () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { pub pose : geometry_msgs :: msg :: PoseWithCovarianceStamped } impl WrappedTypesupport for Request { type CStruct = robot_localization__srv__SetPose_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__robot_localization__srv__SetPose_Request () } } fn create_msg () -> * mut robot_localization__srv__SetPose_Request { unsafe { robot_localization__srv__SetPose_Request__create () } } fn destroy_msg (msg : * mut robot_localization__srv__SetPose_Request) -> () { unsafe { robot_localization__srv__SetPose_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { pose : geometry_msgs :: msg :: PoseWithCovarianceStamped :: from_native (& msg . pose) , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { self . pose . copy_to_native (& mut msg . pose) ; } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { } impl WrappedTypesupport for Response { type CStruct = robot_localization__srv__SetPose_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__robot_localization__srv__SetPose_Response () } } fn create_msg () -> * mut robot_localization__srv__SetPose_Response { unsafe { robot_localization__srv__SetPose_Response__create () } } fn destroy_msg (msg : * mut robot_localization__srv__SetPose_Response) -> () { unsafe { robot_localization__srv__SetPose_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod ToLL { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__robot_localization__srv__ToLL () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { pub map_point : geometry_msgs :: msg :: Point } impl WrappedTypesupport for Request { type CStruct = robot_localization__srv__ToLL_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__robot_localization__srv__ToLL_Request () } } fn create_msg () -> * mut robot_localization__srv__ToLL_Request { unsafe { robot_localization__srv__ToLL_Request__create () } } fn destroy_msg (msg : * mut robot_localization__srv__ToLL_Request) -> () { unsafe { robot_localization__srv__ToLL_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { map_point : geometry_msgs :: msg :: Point :: from_native (& msg . map_point) , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { self . map_point . copy_to_native (& mut msg . map_point) ; } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub ll_point : geographic_msgs :: msg :: GeoPoint } impl WrappedTypesupport for Response { type CStruct = robot_localization__srv__ToLL_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__robot_localization__srv__ToLL_Response () } } fn create_msg () -> * mut robot_localization__srv__ToLL_Response { unsafe { robot_localization__srv__ToLL_Response__create () } } fn destroy_msg (msg : * mut robot_localization__srv__ToLL_Response) -> () { unsafe { robot_localization__srv__ToLL_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { ll_point : geographic_msgs :: msg :: GeoPoint :: from_native (& msg . ll_point) , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { self . ll_point . copy_to_native (& mut msg . ll_point) ; } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod ToggleFilterProcessing { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__robot_localization__srv__ToggleFilterProcessing () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { pub on : bool } impl WrappedTypesupport for Request { type CStruct = robot_localization__srv__ToggleFilterProcessing_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__robot_localization__srv__ToggleFilterProcessing_Request () } } fn create_msg () -> * mut robot_localization__srv__ToggleFilterProcessing_Request { unsafe { robot_localization__srv__ToggleFilterProcessing_Request__create () } } fn destroy_msg (msg : * mut robot_localization__srv__ToggleFilterProcessing_Request) -> () { unsafe { robot_localization__srv__ToggleFilterProcessing_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { on : msg . on , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . on = self . on ; } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub status : bool } impl WrappedTypesupport for Response { type CStruct = robot_localization__srv__ToggleFilterProcessing_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__robot_localization__srv__ToggleFilterProcessing_Response () } } fn create_msg () -> * mut robot_localization__srv__ToggleFilterProcessing_Response { unsafe { robot_localization__srv__ToggleFilterProcessing_Response__create () } } fn destroy_msg (msg : * mut robot_localization__srv__ToggleFilterProcessing_Response) -> () { unsafe { robot_localization__srv__ToggleFilterProcessing_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { status : msg . status , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . status = self . status ; } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } }