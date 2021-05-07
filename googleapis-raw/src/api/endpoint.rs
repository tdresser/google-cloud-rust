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
//! Generated file from `google/api/endpoint.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_23_0;

#[derive(PartialEq,Clone,Default)]
pub struct Endpoint {
    // message fields
    pub name: ::std::string::String,
    pub aliases: ::protobuf::RepeatedField<::std::string::String>,
    pub features: ::protobuf::RepeatedField<::std::string::String>,
    pub target: ::std::string::String,
    pub allow_cors: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Endpoint {
    fn default() -> &'a Endpoint {
        <Endpoint as ::protobuf::Message>::default_instance()
    }
}

impl Endpoint {
    pub fn new() -> Endpoint {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // repeated string aliases = 2;


    pub fn get_aliases(&self) -> &[::std::string::String] {
        &self.aliases
    }
    pub fn clear_aliases(&mut self) {
        self.aliases.clear();
    }

    // Param is passed by value, moved
    pub fn set_aliases(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.aliases = v;
    }

    // Mutable pointer to the field.
    pub fn mut_aliases(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.aliases
    }

    // Take field
    pub fn take_aliases(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.aliases, ::protobuf::RepeatedField::new())
    }

    // repeated string features = 4;


    pub fn get_features(&self) -> &[::std::string::String] {
        &self.features
    }
    pub fn clear_features(&mut self) {
        self.features.clear();
    }

    // Param is passed by value, moved
    pub fn set_features(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.features = v;
    }

    // Mutable pointer to the field.
    pub fn mut_features(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.features
    }

    // Take field
    pub fn take_features(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.features, ::protobuf::RepeatedField::new())
    }

    // string target = 101;


    pub fn get_target(&self) -> &str {
        &self.target
    }
    pub fn clear_target(&mut self) {
        self.target.clear();
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: ::std::string::String) {
        self.target = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&mut self) -> &mut ::std::string::String {
        &mut self.target
    }

    // Take field
    pub fn take_target(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.target, ::std::string::String::new())
    }

    // bool allow_cors = 5;


    pub fn get_allow_cors(&self) -> bool {
        self.allow_cors
    }
    pub fn clear_allow_cors(&mut self) {
        self.allow_cors = false;
    }

    // Param is passed by value, moved
    pub fn set_allow_cors(&mut self, v: bool) {
        self.allow_cors = v;
    }
}

impl ::protobuf::Message for Endpoint {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.aliases)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.features)?;
                },
                101 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.target)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.allow_cors = tmp;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        for value in &self.aliases {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.features {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if !self.target.is_empty() {
            my_size += ::protobuf::rt::string_size(101, &self.target);
        }
        if self.allow_cors != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.aliases {
            os.write_string(2, &v)?;
        };
        for v in &self.features {
            os.write_string(4, &v)?;
        };
        if !self.target.is_empty() {
            os.write_string(101, &self.target)?;
        }
        if self.allow_cors != false {
            os.write_bool(5, self.allow_cors)?;
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

    fn new() -> Endpoint {
        Endpoint::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &Endpoint| { &m.name },
                |m: &mut Endpoint| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "aliases",
                |m: &Endpoint| { &m.aliases },
                |m: &mut Endpoint| { &mut m.aliases },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "features",
                |m: &Endpoint| { &m.features },
                |m: &mut Endpoint| { &mut m.features },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "target",
                |m: &Endpoint| { &m.target },
                |m: &mut Endpoint| { &mut m.target },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "allow_cors",
                |m: &Endpoint| { &m.allow_cors },
                |m: &mut Endpoint| { &mut m.allow_cors },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Endpoint>(
                "Endpoint",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Endpoint {
        static instance: ::protobuf::rt::LazyV2<Endpoint> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Endpoint::new)
    }
}

impl ::protobuf::Clear for Endpoint {
    fn clear(&mut self) {
        self.name.clear();
        self.aliases.clear();
        self.features.clear();
        self.target.clear();
        self.allow_cors = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Endpoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Endpoint {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19google/api/endpoint.proto\x12\ngoogle.api\"\x8f\x01\n\x08Endpoint\
    \x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x1c\n\x07aliases\x18\
    \x02\x20\x03(\tR\x07aliasesB\x02\x18\x01\x12\x1a\n\x08features\x18\x04\
    \x20\x03(\tR\x08features\x12\x16\n\x06target\x18e\x20\x01(\tR\x06target\
    \x12\x1d\n\nallow_cors\x18\x05\x20\x01(\x08R\tallowCorsBo\n\x0ecom.googl\
    e.apiB\rEndpointProtoP\x01ZEgoogle.golang.org/genproto/googleapis/api/se\
    rviceconfig;serviceconfig\xa2\x02\x04GAPIJ\xd3\x15\n\x06\x12\x04\x0f\0E\
    \x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Copyright\x202019\
    \x20Google\x20LLC.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\
    \x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20us\
    e\x20this\x20file\x20except\x20in\x20compliance\x20with\x20the\x20Licens\
    e.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\
    \n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\
    \x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\
    \x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\x20Licen\
    se\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHO\
    UT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20\
    express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20sp\
    ecific\x20language\x20governing\x20permissions\x20and\n\x20limitations\
    \x20under\x20the\x20License.\n\n\n\x08\n\x01\x02\x12\x03\x11\0\x13\n\x08\
    \n\x01\x08\x12\x03\x13\0\\\n\t\n\x02\x08\x0b\x12\x03\x13\0\\\n\x08\n\x01\
    \x08\x12\x03\x14\0\"\n\t\n\x02\x08\n\x12\x03\x14\0\"\n\x08\n\x01\x08\x12\
    \x03\x15\0.\n\t\n\x02\x08\x08\x12\x03\x15\0.\n\x08\n\x01\x08\x12\x03\x16\
    \0'\n\t\n\x02\x08\x01\x12\x03\x16\0'\n\x08\n\x01\x08\x12\x03\x17\0\"\n\t\
    \n\x02\x08$\x12\x03\x17\0\"\n\xce\x05\n\x02\x04\0\x12\x04)\0E\x01\x1a\
    \xc1\x05\x20`Endpoint`\x20describes\x20a\x20network\x20endpoint\x20that\
    \x20serves\x20a\x20set\x20of\x20APIs.\n\x20A\x20service\x20may\x20expose\
    \x20any\x20number\x20of\x20endpoints,\x20and\x20all\x20endpoints\x20shar\
    e\x20the\n\x20same\x20service\x20configuration,\x20such\x20as\x20quota\
    \x20configuration\x20and\x20monitoring\n\x20configuration.\n\n\x20Exampl\
    e\x20service\x20configuration:\n\n\x20\x20\x20\x20\x20name:\x20library-e\
    xample.googleapis.com\n\x20\x20\x20\x20\x20endpoints:\n\x20\x20\x20\x20\
    \x20\x20\x20#\x20Below\x20entry\x20makes\x20'google.example.library.v1.L\
    ibrary'\n\x20\x20\x20\x20\x20\x20\x20#\x20API\x20be\x20served\x20from\
    \x20endpoint\x20address\x20library-example.googleapis.com.\n\x20\x20\x20\
    \x20\x20\x20\x20#\x20It\x20also\x20allows\x20HTTP\x20OPTIONS\x20calls\
    \x20to\x20be\x20passed\x20to\x20the\x20backend,\x20for\n\x20\x20\x20\x20\
    \x20\x20\x20#\x20it\x20to\x20decide\x20whether\x20the\x20subsequent\x20c\
    ross-origin\x20request\x20is\n\x20\x20\x20\x20\x20\x20\x20#\x20allowed\
    \x20to\x20proceed.\n\x20\x20\x20\x20\x20-\x20name:\x20library-example.go\
    ogleapis.com\n\x20\x20\x20\x20\x20\x20\x20allow_cors:\x20true\n\n\n\n\
    \x03\x04\0\x01\x12\x03)\x08\x10\n3\n\x04\x04\0\x02\0\x12\x03+\x02\x12\
    \x1a&\x20The\x20canonical\x20name\x20of\x20this\x20endpoint.\n\n\r\n\x05\
    \x04\0\x02\0\x04\x12\x04+\x02)\x12\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03+\
    \x02\x08\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03+\t\r\n\x0c\n\x05\x04\0\x02\
    \0\x03\x12\x03+\x10\x11\n\xf8\x01\n\x04\x04\0\x02\x01\x12\x032\x022\x1a\
    \xea\x01\x20DEPRECATED:\x20This\x20field\x20is\x20no\x20longer\x20suppor\
    ted.\x20Instead\x20of\x20using\x20aliases,\n\x20please\x20specify\x20mul\
    tiple\x20[google.api.Endpoint][google.api.Endpoint]\x20for\x20each\x20of\
    \x20the\x20intended\n\x20aliases.\n\n\x20Additional\x20names\x20that\x20\
    this\x20endpoint\x20will\x20be\x20hosted\x20on.\n\n\x0c\n\x05\x04\0\x02\
    \x01\x04\x12\x032\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x032\x0b\x11\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x032\x12\x19\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x032\x1c\x1d\n\x0c\n\x05\x04\0\x02\x01\x08\x12\x032\x1e1\n\r\n\
    \x06\x04\0\x02\x01\x08\x03\x12\x032\x1f0\n=\n\x04\x04\0\x02\x02\x12\x035\
    \x02\x1f\x1a0\x20The\x20list\x20of\x20features\x20enabled\x20on\x20this\
    \x20endpoint.\n\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x035\x02\n\n\x0c\n\x05\
    \x04\0\x02\x02\x05\x12\x035\x0b\x11\n\x0c\n\x05\x04\0\x02\x02\x01\x12\
    \x035\x12\x1a\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x035\x1d\x1e\n\xb4\x02\n\
    \x04\x04\0\x02\x03\x12\x03<\x02\x16\x1a\xa6\x02\x20The\x20specification\
    \x20of\x20an\x20Internet\x20routable\x20address\x20of\x20API\x20frontend\
    \x20that\x20will\n\x20handle\x20requests\x20to\x20this\x20[API\n\x20Endp\
    oint](https://cloud.google.com/apis/design/glossary).\x20It\x20should\
    \x20be\n\x20either\x20a\x20valid\x20IPv4\x20address\x20or\x20a\x20fully-\
    qualified\x20domain\x20name.\x20For\x20example,\n\x20\"8.8.8.8\"\x20or\
    \x20\"myservice.appspot.com\".\n\n\r\n\x05\x04\0\x02\x03\x04\x12\x04<\
    \x025\x1f\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03<\x02\x08\n\x0c\n\x05\x04\
    \0\x02\x03\x01\x12\x03<\t\x0f\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03<\x12\
    \x15\n\xd9\x02\n\x04\x04\0\x02\x04\x12\x03D\x02\x16\x1a\xcb\x02\x20Allow\
    ing\n\x20[CORS](https://en.wikipedia.org/wiki/Cross-origin_resource_shar\
    ing),\x20aka\n\x20cross-domain\x20traffic,\x20would\x20allow\x20the\x20b\
    ackends\x20served\x20from\x20this\x20endpoint\x20to\n\x20receive\x20and\
    \x20respond\x20to\x20HTTP\x20OPTIONS\x20requests.\x20The\x20response\x20\
    will\x20be\x20used\x20by\n\x20the\x20browser\x20to\x20determine\x20wheth\
    er\x20the\x20subsequent\x20cross-origin\x20request\x20is\n\x20allowed\
    \x20to\x20proceed.\n\n\r\n\x05\x04\0\x02\x04\x04\x12\x04D\x02<\x16\n\x0c\
    \n\x05\x04\0\x02\x04\x05\x12\x03D\x02\x06\n\x0c\n\x05\x04\0\x02\x04\x01\
    \x12\x03D\x07\x11\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03D\x14\x15b\x06pro\
    to3\
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