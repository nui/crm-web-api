// This file is generated by rust-protobuf 2.25.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `token.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_0;

#[derive(PartialEq,Clone,Default)]
pub struct AccessTokenMsg {
    // message fields
    pub accountId: i64,
    pub lastLogin: i64,
    pub notBefore: i64,
    pub notAfter: i64,
    pub policies: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a AccessTokenMsg {
    fn default() -> &'a AccessTokenMsg {
        <AccessTokenMsg as ::protobuf::Message>::default_instance()
    }
}

impl AccessTokenMsg {
    pub fn new() -> AccessTokenMsg {
        ::std::default::Default::default()
    }

    // int64 accountId = 1;


    pub fn get_accountId(&self) -> i64 {
        self.accountId
    }
    pub fn clear_accountId(&mut self) {
        self.accountId = 0;
    }

    // Param is passed by value, moved
    pub fn set_accountId(&mut self, v: i64) {
        self.accountId = v;
    }

    // int64 lastLogin = 2;


    pub fn get_lastLogin(&self) -> i64 {
        self.lastLogin
    }
    pub fn clear_lastLogin(&mut self) {
        self.lastLogin = 0;
    }

    // Param is passed by value, moved
    pub fn set_lastLogin(&mut self, v: i64) {
        self.lastLogin = v;
    }

    // int64 notBefore = 3;


    pub fn get_notBefore(&self) -> i64 {
        self.notBefore
    }
    pub fn clear_notBefore(&mut self) {
        self.notBefore = 0;
    }

    // Param is passed by value, moved
    pub fn set_notBefore(&mut self, v: i64) {
        self.notBefore = v;
    }

    // int64 notAfter = 4;


    pub fn get_notAfter(&self) -> i64 {
        self.notAfter
    }
    pub fn clear_notAfter(&mut self) {
        self.notAfter = 0;
    }

    // Param is passed by value, moved
    pub fn set_notAfter(&mut self, v: i64) {
        self.notAfter = v;
    }

    // bytes policies = 5;


    pub fn get_policies(&self) -> &[u8] {
        &self.policies
    }
    pub fn clear_policies(&mut self) {
        self.policies.clear();
    }

    // Param is passed by value, moved
    pub fn set_policies(&mut self, v: ::std::vec::Vec<u8>) {
        self.policies = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_policies(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.policies
    }

    // Take field
    pub fn take_policies(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.policies, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for AccessTokenMsg {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.accountId = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.lastLogin = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.notBefore = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.notAfter = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.policies)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.accountId != 0 {
            my_size += ::protobuf::rt::value_size(1, self.accountId, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.lastLogin != 0 {
            my_size += ::protobuf::rt::value_size(2, self.lastLogin, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.notBefore != 0 {
            my_size += ::protobuf::rt::value_size(3, self.notBefore, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.notAfter != 0 {
            my_size += ::protobuf::rt::value_size(4, self.notAfter, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.policies.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.policies);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.accountId != 0 {
            os.write_int64(1, self.accountId)?;
        }
        if self.lastLogin != 0 {
            os.write_int64(2, self.lastLogin)?;
        }
        if self.notBefore != 0 {
            os.write_int64(3, self.notBefore)?;
        }
        if self.notAfter != 0 {
            os.write_int64(4, self.notAfter)?;
        }
        if !self.policies.is_empty() {
            os.write_bytes(5, &self.policies)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> AccessTokenMsg {
        AccessTokenMsg::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "accountId",
                |m: &AccessTokenMsg| { &m.accountId },
                |m: &mut AccessTokenMsg| { &mut m.accountId },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "lastLogin",
                |m: &AccessTokenMsg| { &m.lastLogin },
                |m: &mut AccessTokenMsg| { &mut m.lastLogin },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "notBefore",
                |m: &AccessTokenMsg| { &m.notBefore },
                |m: &mut AccessTokenMsg| { &mut m.notBefore },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "notAfter",
                |m: &AccessTokenMsg| { &m.notAfter },
                |m: &mut AccessTokenMsg| { &mut m.notAfter },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "policies",
                |m: &AccessTokenMsg| { &m.policies },
                |m: &mut AccessTokenMsg| { &mut m.policies },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<AccessTokenMsg>(
                "AccessTokenMsg",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static AccessTokenMsg {
        static instance: ::protobuf::rt::LazyV2<AccessTokenMsg> = ::protobuf::rt::LazyV2::INIT;
        instance.get(AccessTokenMsg::new)
    }
}

impl ::protobuf::Clear for AccessTokenMsg {
    fn clear(&mut self) {
        self.accountId = 0;
        self.lastLogin = 0;
        self.notBefore = 0;
        self.notAfter = 0;
        self.policies.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccessTokenMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccessTokenMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0btoken.proto\"\xae\x01\n\x0eAccessTokenMsg\x12\x1e\n\taccountId\x18\
    \x01\x20\x01(\x03R\taccountIdB\0\x12\x1e\n\tlastLogin\x18\x02\x20\x01(\
    \x03R\tlastLoginB\0\x12\x1e\n\tnotBefore\x18\x03\x20\x01(\x03R\tnotBefor\
    eB\0\x12\x1c\n\x08notAfter\x18\x04\x20\x01(\x03R\x08notAfterB\0\x12\x1c\
    \n\x08policies\x18\x05\x20\x01(\x0cR\x08policiesB\0:\0B\0b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
