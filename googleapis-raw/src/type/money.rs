// This file is generated by rust-protobuf 2.23.0. Do not edit
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
//! Generated file from `google/type/money.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_23_0;

#[derive(PartialEq,Clone,Default)]
pub struct Money {
    // message fields
    pub currency_code: ::std::string::String,
    pub units: i64,
    pub nanos: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Money {
    fn default() -> &'a Money {
        <Money as ::protobuf::Message>::default_instance()
    }
}

impl Money {
    pub fn new() -> Money {
        ::std::default::Default::default()
    }

    // string currency_code = 1;


    pub fn get_currency_code(&self) -> &str {
        &self.currency_code
    }
    pub fn clear_currency_code(&mut self) {
        self.currency_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_currency_code(&mut self, v: ::std::string::String) {
        self.currency_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currency_code(&mut self) -> &mut ::std::string::String {
        &mut self.currency_code
    }

    // Take field
    pub fn take_currency_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.currency_code, ::std::string::String::new())
    }

    // int64 units = 2;


    pub fn get_units(&self) -> i64 {
        self.units
    }
    pub fn clear_units(&mut self) {
        self.units = 0;
    }

    // Param is passed by value, moved
    pub fn set_units(&mut self, v: i64) {
        self.units = v;
    }

    // int32 nanos = 3;


    pub fn get_nanos(&self) -> i32 {
        self.nanos
    }
    pub fn clear_nanos(&mut self) {
        self.nanos = 0;
    }

    // Param is passed by value, moved
    pub fn set_nanos(&mut self, v: i32) {
        self.nanos = v;
    }
}

impl ::protobuf::Message for Money {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.currency_code)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.units = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.nanos = tmp;
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
        if !self.currency_code.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.currency_code);
        }
        if self.units != 0 {
            my_size += ::protobuf::rt::value_size(2, self.units, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.nanos != 0 {
            my_size += ::protobuf::rt::value_size(3, self.nanos, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.currency_code.is_empty() {
            os.write_string(1, &self.currency_code)?;
        }
        if self.units != 0 {
            os.write_int64(2, self.units)?;
        }
        if self.nanos != 0 {
            os.write_int32(3, self.nanos)?;
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

    fn new() -> Money {
        Money::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "currency_code",
                |m: &Money| { &m.currency_code },
                |m: &mut Money| { &mut m.currency_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                "units",
                |m: &Money| { &m.units },
                |m: &mut Money| { &mut m.units },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "nanos",
                |m: &Money| { &m.nanos },
                |m: &mut Money| { &mut m.nanos },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Money>(
                "Money",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Money {
        static instance: ::protobuf::rt::LazyV2<Money> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Money::new)
    }
}

impl ::protobuf::Clear for Money {
    fn clear(&mut self) {
        self.currency_code.clear();
        self.units = 0;
        self.nanos = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Money {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Money {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17google/type/money.proto\x12\x0bgoogle.type\"X\n\x05Money\x12#\n\rc\
    urrency_code\x18\x01\x20\x01(\tR\x0ccurrencyCode\x12\x14\n\x05units\x18\
    \x02\x20\x01(\x03R\x05units\x12\x14\n\x05nanos\x18\x03\x20\x01(\x05R\x05\
    nanosB`\n\x0fcom.google.typeB\nMoneyProtoP\x01Z6google.golang.org/genpro\
    to/googleapis/type/money;money\xf8\x01\x01\xa2\x02\x03GTPJ\x89\x0c\n\x06\
    \x12\x04\x0f\0*\x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Co\
    pyright\x202019\x20Google\x20LLC.\n\n\x20Licensed\x20under\x20the\x20Apa\
    che\x20License,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20m\
    ay\x20not\x20use\x20this\x20file\x20except\x20in\x20compliance\x20with\
    \x20the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\
    \x20License\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/\
    LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\
    \x20agreed\x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\
    \x20the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20B\
    ASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIN\
    D,\x20either\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20\
    for\x20the\x20specific\x20language\x20governing\x20permissions\x20and\n\
    \x20limitations\x20under\x20the\x20License.\n\n\n\x08\n\x01\x02\x12\x03\
    \x11\0\x14\n\x08\n\x01\x08\x12\x03\x13\0\x1f\n\t\n\x02\x08\x1f\x12\x03\
    \x13\0\x1f\n\x08\n\x01\x08\x12\x03\x14\0M\n\t\n\x02\x08\x0b\x12\x03\x14\
    \0M\n\x08\n\x01\x08\x12\x03\x15\0\"\n\t\n\x02\x08\n\x12\x03\x15\0\"\n\
    \x08\n\x01\x08\x12\x03\x16\0+\n\t\n\x02\x08\x08\x12\x03\x16\0+\n\x08\n\
    \x01\x08\x12\x03\x17\0(\n\t\n\x02\x08\x01\x12\x03\x17\0(\n\x08\n\x01\x08\
    \x12\x03\x18\0!\n\t\n\x02\x08$\x12\x03\x18\0!\nC\n\x02\x04\0\x12\x04\x1b\
    \0*\x01\x1a7\x20Represents\x20an\x20amount\x20of\x20money\x20with\x20its\
    \x20currency\x20type.\n\n\n\n\x03\x04\0\x01\x12\x03\x1b\x08\r\n>\n\x04\
    \x04\0\x02\0\x12\x03\x1d\x02\x1b\x1a1\x20The\x203-letter\x20currency\x20\
    code\x20defined\x20in\x20ISO\x204217.\n\n\r\n\x05\x04\0\x02\0\x04\x12\
    \x04\x1d\x02\x1b\x0f\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x1d\x02\x08\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1d\t\x16\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x1d\x19\x1a\nv\n\x04\x04\0\x02\x01\x12\x03!\x02\x12\x1ai\x20The\
    \x20whole\x20units\x20of\x20the\x20amount.\n\x20For\x20example\x20if\x20\
    `currencyCode`\x20is\x20`\"USD\"`,\x20then\x201\x20unit\x20is\x20one\x20\
    US\x20dollar.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04!\x02\x1d\x1b\n\x0c\n\
    \x05\x04\0\x02\x01\x05\x12\x03!\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03!\x08\r\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03!\x10\x11\n\x81\x03\
    \n\x04\x04\0\x02\x02\x12\x03)\x02\x12\x1a\xf3\x02\x20Number\x20of\x20nan\
    o\x20(10^-9)\x20units\x20of\x20the\x20amount.\n\x20The\x20value\x20must\
    \x20be\x20between\x20-999,999,999\x20and\x20+999,999,999\x20inclusive.\n\
    \x20If\x20`units`\x20is\x20positive,\x20`nanos`\x20must\x20be\x20positiv\
    e\x20or\x20zero.\n\x20If\x20`units`\x20is\x20zero,\x20`nanos`\x20can\x20\
    be\x20positive,\x20zero,\x20or\x20negative.\n\x20If\x20`units`\x20is\x20\
    negative,\x20`nanos`\x20must\x20be\x20negative\x20or\x20zero.\n\x20For\
    \x20example\x20$-1.75\x20is\x20represented\x20as\x20`units`=-1\x20and\
    \x20`nanos`=-750,000,000.\n\n\r\n\x05\x04\0\x02\x02\x04\x12\x04)\x02!\
    \x12\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03)\x02\x07\n\x0c\n\x05\x04\0\
    \x02\x02\x01\x12\x03)\x08\r\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03)\x10\
    \x11b\x06proto3\
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
