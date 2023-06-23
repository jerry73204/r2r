pub mod msg { use super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Asset { pub guid : std :: string :: String , # [serde (rename = "type")] pub type_ : std :: string :: String } impl WrappedTypesupport for Asset { type CStruct = rmf_workcell_msgs__msg__Asset ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__rmf_workcell_msgs__msg__Asset () } } fn create_msg () -> * mut rmf_workcell_msgs__msg__Asset { unsafe { rmf_workcell_msgs__msg__Asset__create () } } fn destroy_msg (msg : * mut rmf_workcell_msgs__msg__Asset) -> () { unsafe { rmf_workcell_msgs__msg__Asset__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Asset { Asset { guid : msg . guid . to_str () . to_owned () , type_ : msg . type_ . to_str () . to_owned () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . guid . assign (& self . guid) ; msg . type_ . assign (& self . type_) ; } } impl Default for Asset { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Asset > :: new () ; Asset :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct Trait { pub key : std :: string :: String , pub value : Vec < std :: string :: String > } impl WrappedTypesupport for Trait { type CStruct = rmf_workcell_msgs__msg__Trait ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__rmf_workcell_msgs__msg__Trait () } } fn create_msg () -> * mut rmf_workcell_msgs__msg__Trait { unsafe { rmf_workcell_msgs__msg__Trait__create () } } fn destroy_msg (msg : * mut rmf_workcell_msgs__msg__Trait) -> () { unsafe { rmf_workcell_msgs__msg__Trait__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> Trait { Trait { key : msg . key . to_str () . to_owned () , value : msg . value . to_vec () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . key . assign (& self . key) ; msg . value . update (& self . value) ; } } impl Default for Trait { fn default () -> Self { let msg_native = WrappedNativeMsg :: < Trait > :: new () ; Trait :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct WorkcellConfiguration { pub time : builtin_interfaces :: msg :: Time , pub guid : std :: string :: String , # [serde (rename = "type")] pub type_ : std :: string :: String , pub assets : Vec < rmf_workcell_msgs :: msg :: Asset > , pub traits : Vec < rmf_workcell_msgs :: msg :: Trait > } impl WrappedTypesupport for WorkcellConfiguration { type CStruct = rmf_workcell_msgs__msg__WorkcellConfiguration ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__rmf_workcell_msgs__msg__WorkcellConfiguration () } } fn create_msg () -> * mut rmf_workcell_msgs__msg__WorkcellConfiguration { unsafe { rmf_workcell_msgs__msg__WorkcellConfiguration__create () } } fn destroy_msg (msg : * mut rmf_workcell_msgs__msg__WorkcellConfiguration) -> () { unsafe { rmf_workcell_msgs__msg__WorkcellConfiguration__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> WorkcellConfiguration { WorkcellConfiguration { time : builtin_interfaces :: msg :: Time :: from_native (& msg . time) , guid : msg . guid . to_str () . to_owned () , type_ : msg . type_ . to_str () . to_owned () , assets : { let mut temp = Vec :: with_capacity (msg . assets . size) ; let slice = unsafe { std :: slice :: from_raw_parts (msg . assets . data , msg . assets . size) } ; for s in slice { temp . push (rmf_workcell_msgs :: msg :: Asset :: from_native (s)) ; } temp } , traits : { let mut temp = Vec :: with_capacity (msg . traits . size) ; let slice = unsafe { std :: slice :: from_raw_parts (msg . traits . data , msg . traits . size) } ; for s in slice { temp . push (rmf_workcell_msgs :: msg :: Trait :: from_native (s)) ; } temp } , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { self . time . copy_to_native (& mut msg . time) ; msg . guid . assign (& self . guid) ; msg . type_ . assign (& self . type_) ; unsafe { rmf_workcell_msgs__msg__Asset__Sequence__fini (& mut msg . assets) ; rmf_workcell_msgs__msg__Asset__Sequence__init (& mut msg . assets , self . assets . len ()) ; let slice = std :: slice :: from_raw_parts_mut (msg . assets . data , msg . assets . size) ; for (t , s) in slice . iter_mut () . zip (& self . assets) { s . copy_to_native (t) ; } } unsafe { rmf_workcell_msgs__msg__Trait__Sequence__fini (& mut msg . traits) ; rmf_workcell_msgs__msg__Trait__Sequence__init (& mut msg . traits , self . traits . len ()) ; let slice = std :: slice :: from_raw_parts_mut (msg . traits . data , msg . traits . size) ; for (t , s) in slice . iter_mut () . zip (& self . traits) { s . copy_to_native (t) ; } } } } impl Default for WorkcellConfiguration { fn default () -> Self { let msg_native = WrappedNativeMsg :: < WorkcellConfiguration > :: new () ; WorkcellConfiguration :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct WorkcellRequest { pub time : builtin_interfaces :: msg :: Time , pub request_guid : std :: string :: String , pub target_guid : std :: string :: String } impl WrappedTypesupport for WorkcellRequest { type CStruct = rmf_workcell_msgs__msg__WorkcellRequest ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__rmf_workcell_msgs__msg__WorkcellRequest () } } fn create_msg () -> * mut rmf_workcell_msgs__msg__WorkcellRequest { unsafe { rmf_workcell_msgs__msg__WorkcellRequest__create () } } fn destroy_msg (msg : * mut rmf_workcell_msgs__msg__WorkcellRequest) -> () { unsafe { rmf_workcell_msgs__msg__WorkcellRequest__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> WorkcellRequest { WorkcellRequest { time : builtin_interfaces :: msg :: Time :: from_native (& msg . time) , request_guid : msg . request_guid . to_str () . to_owned () , target_guid : msg . target_guid . to_str () . to_owned () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { self . time . copy_to_native (& mut msg . time) ; msg . request_guid . assign (& self . request_guid) ; msg . target_guid . assign (& self . target_guid) ; } } impl Default for WorkcellRequest { fn default () -> Self { let msg_native = WrappedNativeMsg :: < WorkcellRequest > :: new () ; WorkcellRequest :: from_native (& msg_native) } } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct WorkcellResult { pub time : builtin_interfaces :: msg :: Time , pub request_guid : std :: string :: String , pub source_guid : std :: string :: String , pub status : u8 } impl WrappedTypesupport for WorkcellResult { type CStruct = rmf_workcell_msgs__msg__WorkcellResult ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__rmf_workcell_msgs__msg__WorkcellResult () } } fn create_msg () -> * mut rmf_workcell_msgs__msg__WorkcellResult { unsafe { rmf_workcell_msgs__msg__WorkcellResult__create () } } fn destroy_msg (msg : * mut rmf_workcell_msgs__msg__WorkcellResult) -> () { unsafe { rmf_workcell_msgs__msg__WorkcellResult__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> WorkcellResult { WorkcellResult { time : builtin_interfaces :: msg :: Time :: from_native (& msg . time) , request_guid : msg . request_guid . to_str () . to_owned () , source_guid : msg . source_guid . to_str () . to_owned () , status : msg . status , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { self . time . copy_to_native (& mut msg . time) ; msg . request_guid . assign (& self . request_guid) ; msg . source_guid . assign (& self . source_guid) ; msg . status = self . status ; } } impl Default for WorkcellResult { fn default () -> Self { let msg_native = WrappedNativeMsg :: < WorkcellResult > :: new () ; WorkcellResult :: from_native (& msg_native) } } # [allow (non_upper_case_globals)] impl WorkcellResult { pub const ACKNOWLEDGED : _bindgen_ty_1601 = rmf_workcell_msgs__msg__WorkcellResult__ACKNOWLEDGED ; pub const FAILED : _bindgen_ty_1603 = rmf_workcell_msgs__msg__WorkcellResult__FAILED ; pub const SUCCESS : _bindgen_ty_1602 = rmf_workcell_msgs__msg__WorkcellResult__SUCCESS ; } # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct WorkcellState { pub time : builtin_interfaces :: msg :: Time , pub guid : std :: string :: String , pub mode : i32 , pub request_guid_queue : Vec < std :: string :: String > } impl WrappedTypesupport for WorkcellState { type CStruct = rmf_workcell_msgs__msg__WorkcellState ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__rmf_workcell_msgs__msg__WorkcellState () } } fn create_msg () -> * mut rmf_workcell_msgs__msg__WorkcellState { unsafe { rmf_workcell_msgs__msg__WorkcellState__create () } } fn destroy_msg (msg : * mut rmf_workcell_msgs__msg__WorkcellState) -> () { unsafe { rmf_workcell_msgs__msg__WorkcellState__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> WorkcellState { WorkcellState { time : builtin_interfaces :: msg :: Time :: from_native (& msg . time) , guid : msg . guid . to_str () . to_owned () , mode : msg . mode , request_guid_queue : msg . request_guid_queue . to_vec () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { self . time . copy_to_native (& mut msg . time) ; msg . guid . assign (& self . guid) ; msg . mode = self . mode ; msg . request_guid_queue . update (& self . request_guid_queue) ; } } impl Default for WorkcellState { fn default () -> Self { let msg_native = WrappedNativeMsg :: < WorkcellState > :: new () ; WorkcellState :: from_native (& msg_native) } } # [allow (non_upper_case_globals)] impl WorkcellState { pub const BUSY : _bindgen_ty_1605 = rmf_workcell_msgs__msg__WorkcellState__BUSY ; pub const IDLE : _bindgen_ty_1604 = rmf_workcell_msgs__msg__WorkcellState__IDLE ; pub const OFFLINE : _bindgen_ty_1606 = rmf_workcell_msgs__msg__WorkcellState__OFFLINE ; } }