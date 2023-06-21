// This file is generated by rust-protobuf 3.1.0. Do not edit
// .proto file is parsed by protoc 3.21.9
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

//! Generated file from `protos/CosmosBankSend.proto`

use protobuf::{Error, Message};
use protobuf::well_known_types::any::Any;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:MsgSend)
pub struct MsgSend {
    // message fields
    // @@protoc_insertion_point(field:MsgSend.from_address)
    pub from_address: ::std::string::String,
    // @@protoc_insertion_point(field:MsgSend.to_address)
    pub to_address: ::std::string::String,
    // @@protoc_insertion_point(field:MsgSend.amount)
    pub amount: ::std::vec::Vec<Coin>,
    // special fields
    // @@protoc_insertion_point(special_field:MsgSend.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl MsgSend {
    pub(crate) fn to_any(&self) -> Result<Any, Error> {
        self.write_to_bytes().map(|bytes| Any {
            type_url: "/cosmos.bank.v1beta1.MsgSend".to_string(),
            value: bytes,
            special_fields: Default::default()
        })
    }
}


impl<'a> ::std::default::Default for &'a MsgSend {
    fn default() -> &'a MsgSend {
        <MsgSend as ::protobuf::Message>::default_instance()
    }
}

impl MsgSend {
    pub fn new() -> MsgSend {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "from_address",
            |m: &MsgSend| { &m.from_address },
            |m: &mut MsgSend| { &mut m.from_address },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "to_address",
            |m: &MsgSend| { &m.to_address },
            |m: &mut MsgSend| { &mut m.to_address },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "amount",
            |m: &MsgSend| { &m.amount },
            |m: &mut MsgSend| { &mut m.amount },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MsgSend>(
            "MsgSend",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MsgSend {
    const NAME: &'static str = "MsgSend";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.from_address = is.read_string()?;
                },
                18 => {
                    self.to_address = is.read_string()?;
                },
                26 => {
                    self.amount.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.from_address.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.from_address);
        }
        if !self.to_address.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.to_address);
        }
        for value in &self.amount {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.from_address.is_empty() {
            os.write_string(1, &self.from_address)?;
        }
        if !self.to_address.is_empty() {
            os.write_string(2, &self.to_address)?;
        }
        for v in &self.amount {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MsgSend {
        MsgSend::new()
    }

    fn clear(&mut self) {
        self.from_address.clear();
        self.to_address.clear();
        self.amount.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MsgSend {
        static instance: MsgSend = MsgSend {
            from_address: ::std::string::String::new(),
            to_address: ::std::string::String::new(),
            amount: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MsgSend {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MsgSend").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MsgSend {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MsgSend {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:Coin)
pub struct Coin {
    // message fields
    // @@protoc_insertion_point(field:Coin.denom)
    pub denom: ::std::string::String,
    // @@protoc_insertion_point(field:Coin.amount)
    pub amount: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:Coin.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Coin {
    fn default() -> &'a Coin {
        <Coin as ::protobuf::Message>::default_instance()
    }
}

impl Coin {
    pub fn new() -> Coin {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "denom",
            |m: &Coin| { &m.denom },
            |m: &mut Coin| { &mut m.denom },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "amount",
            |m: &Coin| { &m.amount },
            |m: &mut Coin| { &mut m.amount },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Coin>(
            "Coin",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Coin {
    const NAME: &'static str = "Coin";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.denom = is.read_string()?;
                },
                18 => {
                    self.amount = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.denom.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.denom);
        }
        if !self.amount.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.amount);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.denom.is_empty() {
            os.write_string(1, &self.denom)?;
        }
        if !self.amount.is_empty() {
            os.write_string(2, &self.amount)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Coin {
        Coin::new()
    }

    fn clear(&mut self) {
        self.denom.clear();
        self.amount.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Coin {
        static instance: Coin = Coin {
            denom: ::std::string::String::new(),
            amount: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Coin {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Coin").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Coin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Coin {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bprotos/CosmosBankSend.proto\"j\n\x07MsgSend\x12!\n\x0cfrom_address\
    \x18\x01\x20\x01(\tR\x0bfromAddress\x12\x1d\n\nto_address\x18\x02\x20\
    \x01(\tR\ttoAddress\x12\x1d\n\x06amount\x18\x03\x20\x03(\x0b2\x05.CoinR\
    \x06amount\"4\n\x04Coin\x12\x14\n\x05denom\x18\x01\x20\x01(\tR\x05denom\
    \x12\x16\n\x06amount\x18\x02\x20\x01(\tR\x06amountb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(MsgSend::generated_message_descriptor_data());
            messages.push(Coin::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
