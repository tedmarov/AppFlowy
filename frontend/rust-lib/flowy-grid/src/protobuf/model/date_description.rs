// This file is generated by rust-protobuf 2.25.2. Do not edit
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
//! Generated file from `date_description.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct DateDescription {
    // message fields
    pub date_format: DateFormat,
    pub time_format: TimeFormat,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DateDescription {
    fn default() -> &'a DateDescription {
        <DateDescription as ::protobuf::Message>::default_instance()
    }
}

impl DateDescription {
    pub fn new() -> DateDescription {
        ::std::default::Default::default()
    }

    // .DateFormat date_format = 1;


    pub fn get_date_format(&self) -> DateFormat {
        self.date_format
    }
    pub fn clear_date_format(&mut self) {
        self.date_format = DateFormat::Local;
    }

    // Param is passed by value, moved
    pub fn set_date_format(&mut self, v: DateFormat) {
        self.date_format = v;
    }

    // .TimeFormat time_format = 2;


    pub fn get_time_format(&self) -> TimeFormat {
        self.time_format
    }
    pub fn clear_time_format(&mut self) {
        self.time_format = TimeFormat::TwelveHour;
    }

    // Param is passed by value, moved
    pub fn set_time_format(&mut self, v: TimeFormat) {
        self.time_format = v;
    }
}

impl ::protobuf::Message for DateDescription {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.date_format, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.time_format, 2, &mut self.unknown_fields)?
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
        if self.date_format != DateFormat::Local {
            my_size += ::protobuf::rt::enum_size(1, self.date_format);
        }
        if self.time_format != TimeFormat::TwelveHour {
            my_size += ::protobuf::rt::enum_size(2, self.time_format);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.date_format != DateFormat::Local {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&self.date_format))?;
        }
        if self.time_format != TimeFormat::TwelveHour {
            os.write_enum(2, ::protobuf::ProtobufEnum::value(&self.time_format))?;
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

    fn new() -> DateDescription {
        DateDescription::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DateFormat>>(
                "date_format",
                |m: &DateDescription| { &m.date_format },
                |m: &mut DateDescription| { &mut m.date_format },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<TimeFormat>>(
                "time_format",
                |m: &DateDescription| { &m.time_format },
                |m: &mut DateDescription| { &mut m.time_format },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DateDescription>(
                "DateDescription",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DateDescription {
        static instance: ::protobuf::rt::LazyV2<DateDescription> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DateDescription::new)
    }
}

impl ::protobuf::Clear for DateDescription {
    fn clear(&mut self) {
        self.date_format = DateFormat::Local;
        self.time_format = TimeFormat::TwelveHour;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DateDescription {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DateDescription {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DateFormat {
    Local = 0,
    US = 1,
    ISO = 2,
    Friendly = 3,
}

impl ::protobuf::ProtobufEnum for DateFormat {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DateFormat> {
        match value {
            0 => ::std::option::Option::Some(DateFormat::Local),
            1 => ::std::option::Option::Some(DateFormat::US),
            2 => ::std::option::Option::Some(DateFormat::ISO),
            3 => ::std::option::Option::Some(DateFormat::Friendly),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DateFormat] = &[
            DateFormat::Local,
            DateFormat::US,
            DateFormat::ISO,
            DateFormat::Friendly,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<DateFormat>("DateFormat", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for DateFormat {
}

impl ::std::default::Default for DateFormat {
    fn default() -> Self {
        DateFormat::Local
    }
}

impl ::protobuf::reflect::ProtobufValue for DateFormat {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum TimeFormat {
    TwelveHour = 0,
    TwentyFourHour = 1,
}

impl ::protobuf::ProtobufEnum for TimeFormat {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<TimeFormat> {
        match value {
            0 => ::std::option::Option::Some(TimeFormat::TwelveHour),
            1 => ::std::option::Option::Some(TimeFormat::TwentyFourHour),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [TimeFormat] = &[
            TimeFormat::TwelveHour,
            TimeFormat::TwentyFourHour,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<TimeFormat>("TimeFormat", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for TimeFormat {
}

impl ::std::default::Default for TimeFormat {
    fn default() -> Self {
        TimeFormat::TwelveHour
    }
}

impl ::protobuf::reflect::ProtobufValue for TimeFormat {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16date_description.proto\"m\n\x0fDateDescription\x12,\n\x0bdate_form\
    at\x18\x01\x20\x01(\x0e2\x0b.DateFormatR\ndateFormat\x12,\n\x0btime_form\
    at\x18\x02\x20\x01(\x0e2\x0b.TimeFormatR\ntimeFormat*6\n\nDateFormat\x12\
    \t\n\x05Local\x10\0\x12\x06\n\x02US\x10\x01\x12\x07\n\x03ISO\x10\x02\x12\
    \x0c\n\x08Friendly\x10\x03*0\n\nTimeFormat\x12\x0e\n\nTwelveHour\x10\0\
    \x12\x12\n\x0eTwentyFourHour\x10\x01b\x06proto3\
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
