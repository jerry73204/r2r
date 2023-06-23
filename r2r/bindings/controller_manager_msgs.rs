pub mod srv { # [allow (non_snake_case)] pub mod ConfigureController { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ConfigureController () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { pub name : std :: string :: String } impl WrappedTypesupport for Request { type CStruct = controller_manager_msgs__srv__ConfigureController_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ConfigureController_Request () } } fn create_msg () -> * mut controller_manager_msgs__srv__ConfigureController_Request { unsafe { controller_manager_msgs__srv__ConfigureController_Request__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__ConfigureController_Request) -> () { unsafe { controller_manager_msgs__srv__ConfigureController_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { name : msg . name . to_str () . to_owned () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . name . assign (& self . name) ; } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub ok : bool } impl WrappedTypesupport for Response { type CStruct = controller_manager_msgs__srv__ConfigureController_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ConfigureController_Response () } } fn create_msg () -> * mut controller_manager_msgs__srv__ConfigureController_Response { unsafe { controller_manager_msgs__srv__ConfigureController_Response__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__ConfigureController_Response) -> () { unsafe { controller_manager_msgs__srv__ConfigureController_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { ok : msg . ok , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . ok = self . ok ; } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod ConfigureStartController { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ConfigureStartController () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { pub name : std :: string :: String } impl WrappedTypesupport for Request { type CStruct = controller_manager_msgs__srv__ConfigureStartController_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ConfigureStartController_Request () } } fn create_msg () -> * mut controller_manager_msgs__srv__ConfigureStartController_Request { unsafe { controller_manager_msgs__srv__ConfigureStartController_Request__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__ConfigureStartController_Request) -> () { unsafe { controller_manager_msgs__srv__ConfigureStartController_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { name : msg . name . to_str () . to_owned () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . name . assign (& self . name) ; } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub ok : bool } impl WrappedTypesupport for Response { type CStruct = controller_manager_msgs__srv__ConfigureStartController_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ConfigureStartController_Response () } } fn create_msg () -> * mut controller_manager_msgs__srv__ConfigureStartController_Response { unsafe { controller_manager_msgs__srv__ConfigureStartController_Response__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__ConfigureStartController_Response) -> () { unsafe { controller_manager_msgs__srv__ConfigureStartController_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { ok : msg . ok , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . ok = self . ok ; } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod ListControllerTypes { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ListControllerTypes () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { } impl WrappedTypesupport for Request { type CStruct = controller_manager_msgs__srv__ListControllerTypes_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListControllerTypes_Request () } } fn create_msg () -> * mut controller_manager_msgs__srv__ListControllerTypes_Request { unsafe { controller_manager_msgs__srv__ListControllerTypes_Request__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__ListControllerTypes_Request) -> () { unsafe { controller_manager_msgs__srv__ListControllerTypes_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub types : Vec < std :: string :: String > , pub base_classes : Vec < std :: string :: String > } impl WrappedTypesupport for Response { type CStruct = controller_manager_msgs__srv__ListControllerTypes_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListControllerTypes_Response () } } fn create_msg () -> * mut controller_manager_msgs__srv__ListControllerTypes_Response { unsafe { controller_manager_msgs__srv__ListControllerTypes_Response__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__ListControllerTypes_Response) -> () { unsafe { controller_manager_msgs__srv__ListControllerTypes_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { types : msg . types . to_vec () , base_classes : msg . base_classes . to_vec () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . types . update (& self . types) ; msg . base_classes . update (& self . base_classes) ; } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod ListControllers { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ListControllers () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { } impl WrappedTypesupport for Request { type CStruct = controller_manager_msgs__srv__ListControllers_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListControllers_Request () } } fn create_msg () -> * mut controller_manager_msgs__srv__ListControllers_Request { unsafe { controller_manager_msgs__srv__ListControllers_Request__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__ListControllers_Request) -> () { unsafe { controller_manager_msgs__srv__ListControllers_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub controller : Vec < controller_manager_msgs :: msg :: ControllerState > } impl WrappedTypesupport for Response { type CStruct = controller_manager_msgs__srv__ListControllers_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListControllers_Response () } } fn create_msg () -> * mut controller_manager_msgs__srv__ListControllers_Response { unsafe { controller_manager_msgs__srv__ListControllers_Response__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__ListControllers_Response) -> () { unsafe { controller_manager_msgs__srv__ListControllers_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { controller : { let mut temp = Vec :: with_capacity (msg . controller . size) ; let slice = unsafe { std :: slice :: from_raw_parts (msg . controller . data , msg . controller . size) } ; for s in slice { temp . push (controller_manager_msgs :: msg :: ControllerState :: from_native (s)) ; } temp } , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { unsafe { controller_manager_msgs__msg__ControllerState__Sequence__fini (& mut msg . controller) ; controller_manager_msgs__msg__ControllerState__Sequence__init (& mut msg . controller , self . controller . len ()) ; let slice = std :: slice :: from_raw_parts_mut (msg . controller . data , msg . controller . size) ; for (t , s) in slice . iter_mut () . zip (& self . controller) { s . copy_to_native (t) ; } } } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod ListHardwareInterfaces { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ListHardwareInterfaces () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { } impl WrappedTypesupport for Request { type CStruct = controller_manager_msgs__srv__ListHardwareInterfaces_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListHardwareInterfaces_Request () } } fn create_msg () -> * mut controller_manager_msgs__srv__ListHardwareInterfaces_Request { unsafe { controller_manager_msgs__srv__ListHardwareInterfaces_Request__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__ListHardwareInterfaces_Request) -> () { unsafe { controller_manager_msgs__srv__ListHardwareInterfaces_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub command_interfaces : Vec < controller_manager_msgs :: msg :: HardwareInterface > , pub state_interfaces : Vec < controller_manager_msgs :: msg :: HardwareInterface > } impl WrappedTypesupport for Response { type CStruct = controller_manager_msgs__srv__ListHardwareInterfaces_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ListHardwareInterfaces_Response () } } fn create_msg () -> * mut controller_manager_msgs__srv__ListHardwareInterfaces_Response { unsafe { controller_manager_msgs__srv__ListHardwareInterfaces_Response__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__ListHardwareInterfaces_Response) -> () { unsafe { controller_manager_msgs__srv__ListHardwareInterfaces_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { command_interfaces : { let mut temp = Vec :: with_capacity (msg . command_interfaces . size) ; let slice = unsafe { std :: slice :: from_raw_parts (msg . command_interfaces . data , msg . command_interfaces . size) } ; for s in slice { temp . push (controller_manager_msgs :: msg :: HardwareInterface :: from_native (s)) ; } temp } , state_interfaces : { let mut temp = Vec :: with_capacity (msg . state_interfaces . size) ; let slice = unsafe { std :: slice :: from_raw_parts (msg . state_interfaces . data , msg . state_interfaces . size) } ; for s in slice { temp . push (controller_manager_msgs :: msg :: HardwareInterface :: from_native (s)) ; } temp } , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { unsafe { controller_manager_msgs__msg__HardwareInterface__Sequence__fini (& mut msg . command_interfaces) ; controller_manager_msgs__msg__HardwareInterface__Sequence__init (& mut msg . command_interfaces , self . command_interfaces . len ()) ; let slice = std :: slice :: from_raw_parts_mut (msg . command_interfaces . data , msg . command_interfaces . size) ; for (t , s) in slice . iter_mut () . zip (& self . command_interfaces) { s . copy_to_native (t) ; } } unsafe { controller_manager_msgs__msg__HardwareInterface__Sequence__fini (& mut msg . state_interfaces) ; controller_manager_msgs__msg__HardwareInterface__Sequence__init (& mut msg . state_interfaces , self . state_interfaces . len ()) ; let slice = std :: slice :: from_raw_parts_mut (msg . state_interfaces . data , msg . state_interfaces . size) ; for (t , s) in slice . iter_mut () . zip (& self . state_interfaces) { s . copy_to_native (t) ; } } } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod LoadConfigureController { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__LoadConfigureController () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { pub name : std :: string :: String } impl WrappedTypesupport for Request { type CStruct = controller_manager_msgs__srv__LoadConfigureController_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__LoadConfigureController_Request () } } fn create_msg () -> * mut controller_manager_msgs__srv__LoadConfigureController_Request { unsafe { controller_manager_msgs__srv__LoadConfigureController_Request__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__LoadConfigureController_Request) -> () { unsafe { controller_manager_msgs__srv__LoadConfigureController_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { name : msg . name . to_str () . to_owned () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . name . assign (& self . name) ; } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub ok : bool } impl WrappedTypesupport for Response { type CStruct = controller_manager_msgs__srv__LoadConfigureController_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__LoadConfigureController_Response () } } fn create_msg () -> * mut controller_manager_msgs__srv__LoadConfigureController_Response { unsafe { controller_manager_msgs__srv__LoadConfigureController_Response__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__LoadConfigureController_Response) -> () { unsafe { controller_manager_msgs__srv__LoadConfigureController_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { ok : msg . ok , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . ok = self . ok ; } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod LoadController { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__LoadController () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { pub name : std :: string :: String } impl WrappedTypesupport for Request { type CStruct = controller_manager_msgs__srv__LoadController_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__LoadController_Request () } } fn create_msg () -> * mut controller_manager_msgs__srv__LoadController_Request { unsafe { controller_manager_msgs__srv__LoadController_Request__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__LoadController_Request) -> () { unsafe { controller_manager_msgs__srv__LoadController_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { name : msg . name . to_str () . to_owned () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . name . assign (& self . name) ; } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub ok : bool } impl WrappedTypesupport for Response { type CStruct = controller_manager_msgs__srv__LoadController_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__LoadController_Response () } } fn create_msg () -> * mut controller_manager_msgs__srv__LoadController_Response { unsafe { controller_manager_msgs__srv__LoadController_Response__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__LoadController_Response) -> () { unsafe { controller_manager_msgs__srv__LoadController_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { ok : msg . ok , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . ok = self . ok ; } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod LoadStartController { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__LoadStartController () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { pub name : std :: string :: String } impl WrappedTypesupport for Request { type CStruct = controller_manager_msgs__srv__LoadStartController_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__LoadStartController_Request () } } fn create_msg () -> * mut controller_manager_msgs__srv__LoadStartController_Request { unsafe { controller_manager_msgs__srv__LoadStartController_Request__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__LoadStartController_Request) -> () { unsafe { controller_manager_msgs__srv__LoadStartController_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { name : msg . name . to_str () . to_owned () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . name . assign (& self . name) ; } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub ok : bool } impl WrappedTypesupport for Response { type CStruct = controller_manager_msgs__srv__LoadStartController_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__LoadStartController_Response () } } fn create_msg () -> * mut controller_manager_msgs__srv__LoadStartController_Response { unsafe { controller_manager_msgs__srv__LoadStartController_Response__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__LoadStartController_Response) -> () { unsafe { controller_manager_msgs__srv__LoadStartController_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { ok : msg . ok , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . ok = self . ok ; } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod ReloadControllerLibraries { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__ReloadControllerLibraries () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { pub force_kill : bool } impl WrappedTypesupport for Request { type CStruct = controller_manager_msgs__srv__ReloadControllerLibraries_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ReloadControllerLibraries_Request () } } fn create_msg () -> * mut controller_manager_msgs__srv__ReloadControllerLibraries_Request { unsafe { controller_manager_msgs__srv__ReloadControllerLibraries_Request__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__ReloadControllerLibraries_Request) -> () { unsafe { controller_manager_msgs__srv__ReloadControllerLibraries_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { force_kill : msg . force_kill , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . force_kill = self . force_kill ; } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub ok : bool } impl WrappedTypesupport for Response { type CStruct = controller_manager_msgs__srv__ReloadControllerLibraries_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__ReloadControllerLibraries_Response () } } fn create_msg () -> * mut controller_manager_msgs__srv__ReloadControllerLibraries_Response { unsafe { controller_manager_msgs__srv__ReloadControllerLibraries_Response__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__ReloadControllerLibraries_Response) -> () { unsafe { controller_manager_msgs__srv__ReloadControllerLibraries_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { ok : msg . ok , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . ok = self . ok ; } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod SwitchController { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__SwitchController () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { pub start_controllers : Vec < std :: string :: String > , pub stop_controllers : Vec < std :: string :: String > , pub strictness : i32 , pub start_asap : bool , pub timeout : builtin_interfaces :: msg :: Duration } impl WrappedTypesupport for Request { type CStruct = controller_manager_msgs__srv__SwitchController_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__SwitchController_Request () } } fn create_msg () -> * mut controller_manager_msgs__srv__SwitchController_Request { unsafe { controller_manager_msgs__srv__SwitchController_Request__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__SwitchController_Request) -> () { unsafe { controller_manager_msgs__srv__SwitchController_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { start_controllers : msg . start_controllers . to_vec () , stop_controllers : msg . stop_controllers . to_vec () , strictness : msg . strictness , start_asap : msg . start_asap , timeout : builtin_interfaces :: msg :: Duration :: from_native (& msg . timeout) , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . start_controllers . update (& self . start_controllers) ; msg . stop_controllers . update (& self . stop_controllers) ; msg . strictness = self . strictness ; msg . start_asap = self . start_asap ; self . timeout . copy_to_native (& mut msg . timeout) ; } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [allow (non_upper_case_globals)] impl Request { pub const BEST_EFFORT : _bindgen_ty_160 = controller_manager_msgs__srv__SwitchController_Request__BEST_EFFORT ; pub const STRICT : _bindgen_ty_161 = controller_manager_msgs__srv__SwitchController_Request__STRICT ; } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub ok : bool } impl WrappedTypesupport for Response { type CStruct = controller_manager_msgs__srv__SwitchController_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__SwitchController_Response () } } fn create_msg () -> * mut controller_manager_msgs__srv__SwitchController_Response { unsafe { controller_manager_msgs__srv__SwitchController_Response__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__SwitchController_Response) -> () { unsafe { controller_manager_msgs__srv__SwitchController_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { ok : msg . ok , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . ok = self . ok ; } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } # [allow (non_snake_case)] pub mod UnloadController { use super :: super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] pub struct Service () ; impl WrappedServiceTypeSupport for Service { type Request = Request ; type Response = Response ; fn get_ts () -> & 'static rosidl_service_type_support_t { unsafe { & * rosidl_typesupport_c__get_service_type_support_handle__controller_manager_msgs__srv__UnloadController () } } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Request { pub name : std :: string :: String } impl WrappedTypesupport for Request { type CStruct = controller_manager_msgs__srv__UnloadController_Request ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__UnloadController_Request () } } fn create_msg () -> * mut controller_manager_msgs__srv__UnloadController_Request { unsafe { controller_manager_msgs__srv__UnloadController_Request__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__UnloadController_Request) -> () { unsafe { controller_manager_msgs__srv__UnloadController_Request__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Request { Request { name : msg . name . to_str () . to_owned () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . name . assign (& self . name) ; } } impl Default for Request { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Request > :: new () ; Request :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Response { pub ok : bool } impl WrappedTypesupport for Response { type CStruct = controller_manager_msgs__srv__UnloadController_Response ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__srv__UnloadController_Response () } } fn create_msg () -> * mut controller_manager_msgs__srv__UnloadController_Response { unsafe { controller_manager_msgs__srv__UnloadController_Response__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__srv__UnloadController_Response) -> () { unsafe { controller_manager_msgs__srv__UnloadController_Response__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Response { Response { ok : msg . ok , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . ok = self . ok ; } } impl Default for Response { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Response > :: new () ; Response :: from_native (& msg_native) } } } } pub mod msg { use super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct ControllerState { pub name : std :: string :: String , pub state : std :: string :: String , # [serde (rename = "type")] pub type_ : std :: string :: String , pub claimed_interfaces : Vec < std :: string :: String > , pub required_command_interfaces : Vec < std :: string :: String > , pub required_state_interfaces : Vec < std :: string :: String > } impl WrappedTypesupport for ControllerState { type CStruct = controller_manager_msgs__msg__ControllerState ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__msg__ControllerState () } } fn create_msg () -> * mut controller_manager_msgs__msg__ControllerState { unsafe { controller_manager_msgs__msg__ControllerState__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__msg__ControllerState) -> () { unsafe { controller_manager_msgs__msg__ControllerState__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> ControllerState { ControllerState { name : msg . name . to_str () . to_owned () , state : msg . state . to_str () . to_owned () , type_ : msg . type_ . to_str () . to_owned () , claimed_interfaces : msg . claimed_interfaces . to_vec () , required_command_interfaces : msg . required_command_interfaces . to_vec () , required_state_interfaces : msg . required_state_interfaces . to_vec () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . name . assign (& self . name) ; msg . state . assign (& self . state) ; msg . type_ . assign (& self . type_) ; msg . claimed_interfaces . update (& self . claimed_interfaces) ; msg . required_command_interfaces . update (& self . required_command_interfaces) ; msg . required_state_interfaces . update (& self . required_state_interfaces) ; } } impl Default for ControllerState { fn default () -> Self { let msg_native = WrappedNativeMsg :: < ControllerState > :: new () ; ControllerState :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct HardwareInterface { pub name : std :: string :: String , pub is_claimed : bool } impl WrappedTypesupport for HardwareInterface { type CStruct = controller_manager_msgs__msg__HardwareInterface ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__controller_manager_msgs__msg__HardwareInterface () } } fn create_msg () -> * mut controller_manager_msgs__msg__HardwareInterface { unsafe { controller_manager_msgs__msg__HardwareInterface__create () } } fn destroy_msg (msg : * mut controller_manager_msgs__msg__HardwareInterface) -> () { unsafe { controller_manager_msgs__msg__HardwareInterface__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> HardwareInterface { HardwareInterface { name : msg . name . to_str () . to_owned () , is_claimed : msg . is_claimed , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . name . assign (& self . name) ; msg . is_claimed = self . is_claimed ; } } impl Default for HardwareInterface { fn default () -> Self { let msg_native = WrappedNativeMsg :: < HardwareInterface > :: new () ; HardwareInterface :: from_native (& msg_native) } } }