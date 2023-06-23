pub mod msg { use super :: super :: * ; # [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] # [serde (default)] pub struct CompressedMsg { pub data : Vec < u8 > } impl WrappedTypesupport for CompressedMsg { type CStruct = domain_bridge__msg__CompressedMsg ; fn get_ts () -> & 'static rosidl_message_type_support_t { unsafe { & * rosidl_typesupport_c__get_message_type_support_handle__domain_bridge__msg__CompressedMsg () } } fn create_msg () -> * mut domain_bridge__msg__CompressedMsg { unsafe { domain_bridge__msg__CompressedMsg__create () } } fn destroy_msg (msg : * mut domain_bridge__msg__CompressedMsg) -> () { unsafe { domain_bridge__msg__CompressedMsg__destroy (msg) } ; } fn from_native (# [allow (unused)] msg : & Self :: CStruct) -> CompressedMsg { CompressedMsg { data : msg . data . to_vec () , } } fn copy_to_native (& self , # [allow (unused)] msg : & mut Self :: CStruct) { msg . data . update (& self . data) ; } } impl Default for CompressedMsg { fn default () -> Self { let msg_native = WrappedNativeMsg :: < CompressedMsg > :: new () ; CompressedMsg :: from_native (& msg_native) } } }