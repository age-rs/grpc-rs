// This file is generated by rust-protobuf 3.4.0. Do not edit
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `grpc/testing/payloads.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobufv3::VERSION_3_4_0;

// @@protoc_insertion_point(message:grpc.testing.ByteBufferParams)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ByteBufferParams {
    // message fields
    // @@protoc_insertion_point(field:grpc.testing.ByteBufferParams.req_size)
    pub req_size: i32,
    // @@protoc_insertion_point(field:grpc.testing.ByteBufferParams.resp_size)
    pub resp_size: i32,
    // special fields
    // @@protoc_insertion_point(special_field:grpc.testing.ByteBufferParams.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ByteBufferParams {
    fn default() -> &'a ByteBufferParams {
        <ByteBufferParams as ::protobufv3::Message>::default_instance()
    }
}

impl ByteBufferParams {
    pub fn new() -> ByteBufferParams {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "req_size",
            |m: &ByteBufferParams| { &m.req_size },
            |m: &mut ByteBufferParams| { &mut m.req_size },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "resp_size",
            |m: &ByteBufferParams| { &m.resp_size },
            |m: &mut ByteBufferParams| { &mut m.resp_size },
        ));
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<ByteBufferParams>(
            "ByteBufferParams",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for ByteBufferParams {
    const NAME: &'static str = "ByteBufferParams";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.req_size = is.read_int32()?;
                },
                16 => {
                    self.resp_size = is.read_int32()?;
                },
                tag => {
                    ::protobufv3::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.req_size != 0 {
            my_size += ::protobufv3::rt::int32_size(1, self.req_size);
        }
        if self.resp_size != 0 {
            my_size += ::protobufv3::rt::int32_size(2, self.resp_size);
        }
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        if self.req_size != 0 {
            os.write_int32(1, self.req_size)?;
        }
        if self.resp_size != 0 {
            os.write_int32(2, self.resp_size)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobufv3::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobufv3::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ByteBufferParams {
        ByteBufferParams::new()
    }

    fn clear(&mut self) {
        self.req_size = 0;
        self.resp_size = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ByteBufferParams {
        static instance: ByteBufferParams = ByteBufferParams {
            req_size: 0,
            resp_size: 0,
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for ByteBufferParams {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ByteBufferParams").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ByteBufferParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for ByteBufferParams {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:grpc.testing.SimpleProtoParams)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SimpleProtoParams {
    // message fields
    // @@protoc_insertion_point(field:grpc.testing.SimpleProtoParams.req_size)
    pub req_size: i32,
    // @@protoc_insertion_point(field:grpc.testing.SimpleProtoParams.resp_size)
    pub resp_size: i32,
    // special fields
    // @@protoc_insertion_point(special_field:grpc.testing.SimpleProtoParams.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SimpleProtoParams {
    fn default() -> &'a SimpleProtoParams {
        <SimpleProtoParams as ::protobufv3::Message>::default_instance()
    }
}

impl SimpleProtoParams {
    pub fn new() -> SimpleProtoParams {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "req_size",
            |m: &SimpleProtoParams| { &m.req_size },
            |m: &mut SimpleProtoParams| { &mut m.req_size },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "resp_size",
            |m: &SimpleProtoParams| { &m.resp_size },
            |m: &mut SimpleProtoParams| { &mut m.resp_size },
        ));
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<SimpleProtoParams>(
            "SimpleProtoParams",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for SimpleProtoParams {
    const NAME: &'static str = "SimpleProtoParams";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.req_size = is.read_int32()?;
                },
                16 => {
                    self.resp_size = is.read_int32()?;
                },
                tag => {
                    ::protobufv3::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.req_size != 0 {
            my_size += ::protobufv3::rt::int32_size(1, self.req_size);
        }
        if self.resp_size != 0 {
            my_size += ::protobufv3::rt::int32_size(2, self.resp_size);
        }
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        if self.req_size != 0 {
            os.write_int32(1, self.req_size)?;
        }
        if self.resp_size != 0 {
            os.write_int32(2, self.resp_size)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobufv3::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobufv3::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SimpleProtoParams {
        SimpleProtoParams::new()
    }

    fn clear(&mut self) {
        self.req_size = 0;
        self.resp_size = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SimpleProtoParams {
        static instance: SimpleProtoParams = SimpleProtoParams {
            req_size: 0,
            resp_size: 0,
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for SimpleProtoParams {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SimpleProtoParams").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SimpleProtoParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for SimpleProtoParams {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:grpc.testing.ComplexProtoParams)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ComplexProtoParams {
    // special fields
    // @@protoc_insertion_point(special_field:grpc.testing.ComplexProtoParams.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ComplexProtoParams {
    fn default() -> &'a ComplexProtoParams {
        <ComplexProtoParams as ::protobufv3::Message>::default_instance()
    }
}

impl ComplexProtoParams {
    pub fn new() -> ComplexProtoParams {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(0);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<ComplexProtoParams>(
            "ComplexProtoParams",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for ComplexProtoParams {
    const NAME: &'static str = "ComplexProtoParams";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                tag => {
                    ::protobufv3::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobufv3::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobufv3::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ComplexProtoParams {
        ComplexProtoParams::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ComplexProtoParams {
        static instance: ComplexProtoParams = ComplexProtoParams {
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for ComplexProtoParams {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ComplexProtoParams").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ComplexProtoParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for ComplexProtoParams {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:grpc.testing.PayloadConfig)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PayloadConfig {
    // message oneof groups
    pub payload: ::std::option::Option<payload_config::Payload>,
    // special fields
    // @@protoc_insertion_point(special_field:grpc.testing.PayloadConfig.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PayloadConfig {
    fn default() -> &'a PayloadConfig {
        <PayloadConfig as ::protobufv3::Message>::default_instance()
    }
}

impl PayloadConfig {
    pub fn new() -> PayloadConfig {
        ::std::default::Default::default()
    }

    // .grpc.testing.ByteBufferParams bytebuf_params = 1;

    pub fn bytebuf_params(&self) -> &ByteBufferParams {
        match self.payload {
            ::std::option::Option::Some(payload_config::Payload::BytebufParams(ref v)) => v,
            _ => <ByteBufferParams as ::protobufv3::Message>::default_instance(),
        }
    }

    pub fn clear_bytebuf_params(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_bytebuf_params(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(payload_config::Payload::BytebufParams(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bytebuf_params(&mut self, v: ByteBufferParams) {
        self.payload = ::std::option::Option::Some(payload_config::Payload::BytebufParams(v))
    }

    // Mutable pointer to the field.
    pub fn mut_bytebuf_params(&mut self) -> &mut ByteBufferParams {
        if let ::std::option::Option::Some(payload_config::Payload::BytebufParams(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(payload_config::Payload::BytebufParams(ByteBufferParams::new()));
        }
        match self.payload {
            ::std::option::Option::Some(payload_config::Payload::BytebufParams(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_bytebuf_params(&mut self) -> ByteBufferParams {
        if self.has_bytebuf_params() {
            match self.payload.take() {
                ::std::option::Option::Some(payload_config::Payload::BytebufParams(v)) => v,
                _ => panic!(),
            }
        } else {
            ByteBufferParams::new()
        }
    }

    // .grpc.testing.SimpleProtoParams simple_params = 2;

    pub fn simple_params(&self) -> &SimpleProtoParams {
        match self.payload {
            ::std::option::Option::Some(payload_config::Payload::SimpleParams(ref v)) => v,
            _ => <SimpleProtoParams as ::protobufv3::Message>::default_instance(),
        }
    }

    pub fn clear_simple_params(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_simple_params(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(payload_config::Payload::SimpleParams(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_simple_params(&mut self, v: SimpleProtoParams) {
        self.payload = ::std::option::Option::Some(payload_config::Payload::SimpleParams(v))
    }

    // Mutable pointer to the field.
    pub fn mut_simple_params(&mut self) -> &mut SimpleProtoParams {
        if let ::std::option::Option::Some(payload_config::Payload::SimpleParams(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(payload_config::Payload::SimpleParams(SimpleProtoParams::new()));
        }
        match self.payload {
            ::std::option::Option::Some(payload_config::Payload::SimpleParams(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_simple_params(&mut self) -> SimpleProtoParams {
        if self.has_simple_params() {
            match self.payload.take() {
                ::std::option::Option::Some(payload_config::Payload::SimpleParams(v)) => v,
                _ => panic!(),
            }
        } else {
            SimpleProtoParams::new()
        }
    }

    // .grpc.testing.ComplexProtoParams complex_params = 3;

    pub fn complex_params(&self) -> &ComplexProtoParams {
        match self.payload {
            ::std::option::Option::Some(payload_config::Payload::ComplexParams(ref v)) => v,
            _ => <ComplexProtoParams as ::protobufv3::Message>::default_instance(),
        }
    }

    pub fn clear_complex_params(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_complex_params(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(payload_config::Payload::ComplexParams(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_complex_params(&mut self, v: ComplexProtoParams) {
        self.payload = ::std::option::Option::Some(payload_config::Payload::ComplexParams(v))
    }

    // Mutable pointer to the field.
    pub fn mut_complex_params(&mut self) -> &mut ComplexProtoParams {
        if let ::std::option::Option::Some(payload_config::Payload::ComplexParams(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(payload_config::Payload::ComplexParams(ComplexProtoParams::new()));
        }
        match self.payload {
            ::std::option::Option::Some(payload_config::Payload::ComplexParams(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_complex_params(&mut self) -> ComplexProtoParams {
        if self.has_complex_params() {
            match self.payload.take() {
                ::std::option::Option::Some(payload_config::Payload::ComplexParams(v)) => v,
                _ => panic!(),
            }
        } else {
            ComplexProtoParams::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobufv3::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, ByteBufferParams>(
            "bytebuf_params",
            PayloadConfig::has_bytebuf_params,
            PayloadConfig::bytebuf_params,
            PayloadConfig::mut_bytebuf_params,
            PayloadConfig::set_bytebuf_params,
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, SimpleProtoParams>(
            "simple_params",
            PayloadConfig::has_simple_params,
            PayloadConfig::simple_params,
            PayloadConfig::mut_simple_params,
            PayloadConfig::set_simple_params,
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, ComplexProtoParams>(
            "complex_params",
            PayloadConfig::has_complex_params,
            PayloadConfig::complex_params,
            PayloadConfig::mut_complex_params,
            PayloadConfig::set_complex_params,
        ));
        oneofs.push(payload_config::Payload::generated_oneof_descriptor_data());
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<PayloadConfig>(
            "PayloadConfig",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for PayloadConfig {
    const NAME: &'static str = "PayloadConfig";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.payload = ::std::option::Option::Some(payload_config::Payload::BytebufParams(is.read_message()?));
                },
                18 => {
                    self.payload = ::std::option::Option::Some(payload_config::Payload::SimpleParams(is.read_message()?));
                },
                26 => {
                    self.payload = ::std::option::Option::Some(payload_config::Payload::ComplexParams(is.read_message()?));
                },
                tag => {
                    ::protobufv3::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &payload_config::Payload::BytebufParams(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobufv3::rt::compute_raw_varint64_size(len) + len;
                },
                &payload_config::Payload::SimpleParams(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobufv3::rt::compute_raw_varint64_size(len) + len;
                },
                &payload_config::Payload::ComplexParams(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobufv3::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &payload_config::Payload::BytebufParams(ref v) => {
                    ::protobufv3::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &payload_config::Payload::SimpleParams(ref v) => {
                    ::protobufv3::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &payload_config::Payload::ComplexParams(ref v) => {
                    ::protobufv3::rt::write_message_field_with_cached_size(3, v, os)?;
                },
            };
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobufv3::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobufv3::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PayloadConfig {
        PayloadConfig::new()
    }

    fn clear(&mut self) {
        self.payload = ::std::option::Option::None;
        self.payload = ::std::option::Option::None;
        self.payload = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PayloadConfig {
        static instance: PayloadConfig = PayloadConfig {
            payload: ::std::option::Option::None,
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for PayloadConfig {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PayloadConfig").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PayloadConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for PayloadConfig {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `PayloadConfig`
pub mod payload_config {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:grpc.testing.PayloadConfig.payload)
    pub enum Payload {
        // @@protoc_insertion_point(oneof_field:grpc.testing.PayloadConfig.bytebuf_params)
        BytebufParams(super::ByteBufferParams),
        // @@protoc_insertion_point(oneof_field:grpc.testing.PayloadConfig.simple_params)
        SimpleParams(super::SimpleProtoParams),
        // @@protoc_insertion_point(oneof_field:grpc.testing.PayloadConfig.complex_params)
        ComplexParams(super::ComplexProtoParams),
    }

    impl ::protobufv3::Oneof for Payload {
    }

    impl ::protobufv3::OneofFull for Payload {
        fn descriptor() -> ::protobufv3::reflect::OneofDescriptor {
            static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::OneofDescriptor> = ::protobufv3::rt::Lazy::new();
            descriptor.get(|| <super::PayloadConfig as ::protobufv3::MessageFull>::descriptor().oneof_by_name("payload").unwrap()).clone()
        }
    }

    impl Payload {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobufv3::reflect::GeneratedOneofDescriptorData {
            ::protobufv3::reflect::GeneratedOneofDescriptorData::new::<Payload>("payload")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgrpc/testing/payloads.proto\x12\x0cgrpc.testing\"J\n\x10ByteBuffer\
    Params\x12\x19\n\x08req_size\x18\x01\x20\x01(\x05R\x07reqSize\x12\x1b\n\
    \tresp_size\x18\x02\x20\x01(\x05R\x08respSize\"K\n\x11SimpleProtoParams\
    \x12\x19\n\x08req_size\x18\x01\x20\x01(\x05R\x07reqSize\x12\x1b\n\tresp_\
    size\x18\x02\x20\x01(\x05R\x08respSize\"\x14\n\x12ComplexProtoParams\"\
    \xf6\x01\n\rPayloadConfig\x12G\n\x0ebytebuf_params\x18\x01\x20\x01(\x0b2\
    \x1e.grpc.testing.ByteBufferParamsH\0R\rbytebufParams\x12F\n\rsimple_par\
    ams\x18\x02\x20\x01(\x0b2\x1f.grpc.testing.SimpleProtoParamsH\0R\x0csimp\
    leParams\x12I\n\x0ecomplex_params\x18\x03\x20\x01(\x0b2\x20.grpc.testing\
    .ComplexProtoParamsH\0R\rcomplexParamsB\t\n\x07payloadb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobufv3::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobufv3::rt::Lazy<::protobufv3::descriptor::FileDescriptorProto> = ::protobufv3::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobufv3::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobufv3::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobufv3::rt::Lazy<::protobufv3::reflect::GeneratedFileDescriptor> = ::protobufv3::rt::Lazy::new();
    static file_descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::FileDescriptor> = ::protobufv3::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(ByteBufferParams::generated_message_descriptor_data());
            messages.push(SimpleProtoParams::generated_message_descriptor_data());
            messages.push(ComplexProtoParams::generated_message_descriptor_data());
            messages.push(PayloadConfig::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobufv3::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobufv3::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}