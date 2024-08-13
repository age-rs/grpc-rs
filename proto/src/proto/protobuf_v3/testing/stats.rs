// This file is generated by rust-protobuf 3.5.0. Do not edit
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

//! Generated file from `grpc/testing/stats.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobufv3::VERSION_3_5_0;

// @@protoc_insertion_point(message:grpc.testing.ServerStats)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ServerStats {
    // message fields
    // @@protoc_insertion_point(field:grpc.testing.ServerStats.time_elapsed)
    pub time_elapsed: f64,
    // @@protoc_insertion_point(field:grpc.testing.ServerStats.time_user)
    pub time_user: f64,
    // @@protoc_insertion_point(field:grpc.testing.ServerStats.time_system)
    pub time_system: f64,
    // @@protoc_insertion_point(field:grpc.testing.ServerStats.total_cpu_time)
    pub total_cpu_time: u64,
    // @@protoc_insertion_point(field:grpc.testing.ServerStats.idle_cpu_time)
    pub idle_cpu_time: u64,
    // @@protoc_insertion_point(field:grpc.testing.ServerStats.cq_poll_count)
    pub cq_poll_count: u64,
    // special fields
    // @@protoc_insertion_point(special_field:grpc.testing.ServerStats.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ServerStats {
    fn default() -> &'a ServerStats {
        <ServerStats as ::protobufv3::Message>::default_instance()
    }
}

impl ServerStats {
    pub fn new() -> ServerStats {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time_elapsed",
            |m: &ServerStats| { &m.time_elapsed },
            |m: &mut ServerStats| { &mut m.time_elapsed },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time_user",
            |m: &ServerStats| { &m.time_user },
            |m: &mut ServerStats| { &mut m.time_user },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time_system",
            |m: &ServerStats| { &m.time_system },
            |m: &mut ServerStats| { &mut m.time_system },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "total_cpu_time",
            |m: &ServerStats| { &m.total_cpu_time },
            |m: &mut ServerStats| { &mut m.total_cpu_time },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "idle_cpu_time",
            |m: &ServerStats| { &m.idle_cpu_time },
            |m: &mut ServerStats| { &mut m.idle_cpu_time },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cq_poll_count",
            |m: &ServerStats| { &m.cq_poll_count },
            |m: &mut ServerStats| { &mut m.cq_poll_count },
        ));
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<ServerStats>(
            "ServerStats",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for ServerStats {
    const NAME: &'static str = "ServerStats";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                9 => {
                    self.time_elapsed = is.read_double()?;
                },
                17 => {
                    self.time_user = is.read_double()?;
                },
                25 => {
                    self.time_system = is.read_double()?;
                },
                32 => {
                    self.total_cpu_time = is.read_uint64()?;
                },
                40 => {
                    self.idle_cpu_time = is.read_uint64()?;
                },
                48 => {
                    self.cq_poll_count = is.read_uint64()?;
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
        if self.time_elapsed != 0. {
            my_size += 1 + 8;
        }
        if self.time_user != 0. {
            my_size += 1 + 8;
        }
        if self.time_system != 0. {
            my_size += 1 + 8;
        }
        if self.total_cpu_time != 0 {
            my_size += ::protobufv3::rt::uint64_size(4, self.total_cpu_time);
        }
        if self.idle_cpu_time != 0 {
            my_size += ::protobufv3::rt::uint64_size(5, self.idle_cpu_time);
        }
        if self.cq_poll_count != 0 {
            my_size += ::protobufv3::rt::uint64_size(6, self.cq_poll_count);
        }
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        if self.time_elapsed != 0. {
            os.write_double(1, self.time_elapsed)?;
        }
        if self.time_user != 0. {
            os.write_double(2, self.time_user)?;
        }
        if self.time_system != 0. {
            os.write_double(3, self.time_system)?;
        }
        if self.total_cpu_time != 0 {
            os.write_uint64(4, self.total_cpu_time)?;
        }
        if self.idle_cpu_time != 0 {
            os.write_uint64(5, self.idle_cpu_time)?;
        }
        if self.cq_poll_count != 0 {
            os.write_uint64(6, self.cq_poll_count)?;
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

    fn new() -> ServerStats {
        ServerStats::new()
    }

    fn clear(&mut self) {
        self.time_elapsed = 0.;
        self.time_user = 0.;
        self.time_system = 0.;
        self.total_cpu_time = 0;
        self.idle_cpu_time = 0;
        self.cq_poll_count = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ServerStats {
        static instance: ServerStats = ServerStats {
            time_elapsed: 0.,
            time_user: 0.,
            time_system: 0.,
            total_cpu_time: 0,
            idle_cpu_time: 0,
            cq_poll_count: 0,
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for ServerStats {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ServerStats").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ServerStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for ServerStats {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:grpc.testing.HistogramParams)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HistogramParams {
    // message fields
    // @@protoc_insertion_point(field:grpc.testing.HistogramParams.resolution)
    pub resolution: f64,
    // @@protoc_insertion_point(field:grpc.testing.HistogramParams.max_possible)
    pub max_possible: f64,
    // special fields
    // @@protoc_insertion_point(special_field:grpc.testing.HistogramParams.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HistogramParams {
    fn default() -> &'a HistogramParams {
        <HistogramParams as ::protobufv3::Message>::default_instance()
    }
}

impl HistogramParams {
    pub fn new() -> HistogramParams {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "resolution",
            |m: &HistogramParams| { &m.resolution },
            |m: &mut HistogramParams| { &mut m.resolution },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_possible",
            |m: &HistogramParams| { &m.max_possible },
            |m: &mut HistogramParams| { &mut m.max_possible },
        ));
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<HistogramParams>(
            "HistogramParams",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for HistogramParams {
    const NAME: &'static str = "HistogramParams";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                9 => {
                    self.resolution = is.read_double()?;
                },
                17 => {
                    self.max_possible = is.read_double()?;
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
        if self.resolution != 0. {
            my_size += 1 + 8;
        }
        if self.max_possible != 0. {
            my_size += 1 + 8;
        }
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        if self.resolution != 0. {
            os.write_double(1, self.resolution)?;
        }
        if self.max_possible != 0. {
            os.write_double(2, self.max_possible)?;
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

    fn new() -> HistogramParams {
        HistogramParams::new()
    }

    fn clear(&mut self) {
        self.resolution = 0.;
        self.max_possible = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HistogramParams {
        static instance: HistogramParams = HistogramParams {
            resolution: 0.,
            max_possible: 0.,
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for HistogramParams {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HistogramParams").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HistogramParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for HistogramParams {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:grpc.testing.HistogramData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HistogramData {
    // message fields
    // @@protoc_insertion_point(field:grpc.testing.HistogramData.bucket)
    pub bucket: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:grpc.testing.HistogramData.min_seen)
    pub min_seen: f64,
    // @@protoc_insertion_point(field:grpc.testing.HistogramData.max_seen)
    pub max_seen: f64,
    // @@protoc_insertion_point(field:grpc.testing.HistogramData.sum)
    pub sum: f64,
    // @@protoc_insertion_point(field:grpc.testing.HistogramData.sum_of_squares)
    pub sum_of_squares: f64,
    // @@protoc_insertion_point(field:grpc.testing.HistogramData.count)
    pub count: f64,
    // special fields
    // @@protoc_insertion_point(special_field:grpc.testing.HistogramData.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HistogramData {
    fn default() -> &'a HistogramData {
        <HistogramData as ::protobufv3::Message>::default_instance()
    }
}

impl HistogramData {
    pub fn new() -> HistogramData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobufv3::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "bucket",
            |m: &HistogramData| { &m.bucket },
            |m: &mut HistogramData| { &mut m.bucket },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "min_seen",
            |m: &HistogramData| { &m.min_seen },
            |m: &mut HistogramData| { &mut m.min_seen },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_seen",
            |m: &HistogramData| { &m.max_seen },
            |m: &mut HistogramData| { &mut m.max_seen },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sum",
            |m: &HistogramData| { &m.sum },
            |m: &mut HistogramData| { &mut m.sum },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sum_of_squares",
            |m: &HistogramData| { &m.sum_of_squares },
            |m: &mut HistogramData| { &mut m.sum_of_squares },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "count",
            |m: &HistogramData| { &m.count },
            |m: &mut HistogramData| { &mut m.count },
        ));
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<HistogramData>(
            "HistogramData",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for HistogramData {
    const NAME: &'static str = "HistogramData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.bucket)?;
                },
                8 => {
                    self.bucket.push(is.read_uint32()?);
                },
                17 => {
                    self.min_seen = is.read_double()?;
                },
                25 => {
                    self.max_seen = is.read_double()?;
                },
                33 => {
                    self.sum = is.read_double()?;
                },
                41 => {
                    self.sum_of_squares = is.read_double()?;
                },
                49 => {
                    self.count = is.read_double()?;
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
        my_size += ::protobufv3::rt::vec_packed_uint32_size(1, &self.bucket);
        if self.min_seen != 0. {
            my_size += 1 + 8;
        }
        if self.max_seen != 0. {
            my_size += 1 + 8;
        }
        if self.sum != 0. {
            my_size += 1 + 8;
        }
        if self.sum_of_squares != 0. {
            my_size += 1 + 8;
        }
        if self.count != 0. {
            my_size += 1 + 8;
        }
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        os.write_repeated_packed_uint32(1, &self.bucket)?;
        if self.min_seen != 0. {
            os.write_double(2, self.min_seen)?;
        }
        if self.max_seen != 0. {
            os.write_double(3, self.max_seen)?;
        }
        if self.sum != 0. {
            os.write_double(4, self.sum)?;
        }
        if self.sum_of_squares != 0. {
            os.write_double(5, self.sum_of_squares)?;
        }
        if self.count != 0. {
            os.write_double(6, self.count)?;
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

    fn new() -> HistogramData {
        HistogramData::new()
    }

    fn clear(&mut self) {
        self.bucket.clear();
        self.min_seen = 0.;
        self.max_seen = 0.;
        self.sum = 0.;
        self.sum_of_squares = 0.;
        self.count = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HistogramData {
        static instance: HistogramData = HistogramData {
            bucket: ::std::vec::Vec::new(),
            min_seen: 0.,
            max_seen: 0.,
            sum: 0.,
            sum_of_squares: 0.,
            count: 0.,
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for HistogramData {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HistogramData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HistogramData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for HistogramData {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:grpc.testing.RequestResultCount)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RequestResultCount {
    // message fields
    // @@protoc_insertion_point(field:grpc.testing.RequestResultCount.status_code)
    pub status_code: i32,
    // @@protoc_insertion_point(field:grpc.testing.RequestResultCount.count)
    pub count: i64,
    // special fields
    // @@protoc_insertion_point(special_field:grpc.testing.RequestResultCount.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RequestResultCount {
    fn default() -> &'a RequestResultCount {
        <RequestResultCount as ::protobufv3::Message>::default_instance()
    }
}

impl RequestResultCount {
    pub fn new() -> RequestResultCount {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status_code",
            |m: &RequestResultCount| { &m.status_code },
            |m: &mut RequestResultCount| { &mut m.status_code },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "count",
            |m: &RequestResultCount| { &m.count },
            |m: &mut RequestResultCount| { &mut m.count },
        ));
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<RequestResultCount>(
            "RequestResultCount",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for RequestResultCount {
    const NAME: &'static str = "RequestResultCount";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.status_code = is.read_int32()?;
                },
                16 => {
                    self.count = is.read_int64()?;
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
        if self.status_code != 0 {
            my_size += ::protobufv3::rt::int32_size(1, self.status_code);
        }
        if self.count != 0 {
            my_size += ::protobufv3::rt::int64_size(2, self.count);
        }
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        if self.status_code != 0 {
            os.write_int32(1, self.status_code)?;
        }
        if self.count != 0 {
            os.write_int64(2, self.count)?;
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

    fn new() -> RequestResultCount {
        RequestResultCount::new()
    }

    fn clear(&mut self) {
        self.status_code = 0;
        self.count = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RequestResultCount {
        static instance: RequestResultCount = RequestResultCount {
            status_code: 0,
            count: 0,
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for RequestResultCount {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RequestResultCount").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RequestResultCount {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for RequestResultCount {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:grpc.testing.ClientStats)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ClientStats {
    // message fields
    // @@protoc_insertion_point(field:grpc.testing.ClientStats.latencies)
    pub latencies: ::protobufv3::MessageField<HistogramData>,
    // @@protoc_insertion_point(field:grpc.testing.ClientStats.time_elapsed)
    pub time_elapsed: f64,
    // @@protoc_insertion_point(field:grpc.testing.ClientStats.time_user)
    pub time_user: f64,
    // @@protoc_insertion_point(field:grpc.testing.ClientStats.time_system)
    pub time_system: f64,
    // @@protoc_insertion_point(field:grpc.testing.ClientStats.request_results)
    pub request_results: ::std::vec::Vec<RequestResultCount>,
    // @@protoc_insertion_point(field:grpc.testing.ClientStats.cq_poll_count)
    pub cq_poll_count: u64,
    // special fields
    // @@protoc_insertion_point(special_field:grpc.testing.ClientStats.special_fields)
    pub special_fields: ::protobufv3::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ClientStats {
    fn default() -> &'a ClientStats {
        <ClientStats as ::protobufv3::Message>::default_instance()
    }
}

impl ClientStats {
    pub fn new() -> ClientStats {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobufv3::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobufv3::reflect::rt::v2::make_message_field_accessor::<_, HistogramData>(
            "latencies",
            |m: &ClientStats| { &m.latencies },
            |m: &mut ClientStats| { &mut m.latencies },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time_elapsed",
            |m: &ClientStats| { &m.time_elapsed },
            |m: &mut ClientStats| { &mut m.time_elapsed },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time_user",
            |m: &ClientStats| { &m.time_user },
            |m: &mut ClientStats| { &mut m.time_user },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time_system",
            |m: &ClientStats| { &m.time_system },
            |m: &mut ClientStats| { &mut m.time_system },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "request_results",
            |m: &ClientStats| { &m.request_results },
            |m: &mut ClientStats| { &mut m.request_results },
        ));
        fields.push(::protobufv3::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cq_poll_count",
            |m: &ClientStats| { &m.cq_poll_count },
            |m: &mut ClientStats| { &mut m.cq_poll_count },
        ));
        ::protobufv3::reflect::GeneratedMessageDescriptorData::new_2::<ClientStats>(
            "ClientStats",
            fields,
            oneofs,
        )
    }
}

impl ::protobufv3::Message for ClientStats {
    const NAME: &'static str = "ClientStats";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobufv3::CodedInputStream<'_>) -> ::protobufv3::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobufv3::rt::read_singular_message_into_field(is, &mut self.latencies)?;
                },
                17 => {
                    self.time_elapsed = is.read_double()?;
                },
                25 => {
                    self.time_user = is.read_double()?;
                },
                33 => {
                    self.time_system = is.read_double()?;
                },
                42 => {
                    self.request_results.push(is.read_message()?);
                },
                48 => {
                    self.cq_poll_count = is.read_uint64()?;
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
        if let Some(v) = self.latencies.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobufv3::rt::compute_raw_varint64_size(len) + len;
        }
        if self.time_elapsed != 0. {
            my_size += 1 + 8;
        }
        if self.time_user != 0. {
            my_size += 1 + 8;
        }
        if self.time_system != 0. {
            my_size += 1 + 8;
        }
        for value in &self.request_results {
            let len = value.compute_size();
            my_size += 1 + ::protobufv3::rt::compute_raw_varint64_size(len) + len;
        };
        if self.cq_poll_count != 0 {
            my_size += ::protobufv3::rt::uint64_size(6, self.cq_poll_count);
        }
        my_size += ::protobufv3::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobufv3::CodedOutputStream<'_>) -> ::protobufv3::Result<()> {
        if let Some(v) = self.latencies.as_ref() {
            ::protobufv3::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.time_elapsed != 0. {
            os.write_double(2, self.time_elapsed)?;
        }
        if self.time_user != 0. {
            os.write_double(3, self.time_user)?;
        }
        if self.time_system != 0. {
            os.write_double(4, self.time_system)?;
        }
        for v in &self.request_results {
            ::protobufv3::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.cq_poll_count != 0 {
            os.write_uint64(6, self.cq_poll_count)?;
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

    fn new() -> ClientStats {
        ClientStats::new()
    }

    fn clear(&mut self) {
        self.latencies.clear();
        self.time_elapsed = 0.;
        self.time_user = 0.;
        self.time_system = 0.;
        self.request_results.clear();
        self.cq_poll_count = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ClientStats {
        static instance: ClientStats = ClientStats {
            latencies: ::protobufv3::MessageField::none(),
            time_elapsed: 0.,
            time_user: 0.,
            time_system: 0.,
            request_results: ::std::vec::Vec::new(),
            cq_poll_count: 0,
            special_fields: ::protobufv3::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobufv3::MessageFull for ClientStats {
    fn descriptor() -> ::protobufv3::reflect::MessageDescriptor {
        static descriptor: ::protobufv3::rt::Lazy<::protobufv3::reflect::MessageDescriptor> = ::protobufv3::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ClientStats").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ClientStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobufv3::text_format::fmt(self, f)
    }
}

impl ::protobufv3::reflect::ProtobufValue for ClientStats {
    type RuntimeType = ::protobufv3::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18grpc/testing/stats.proto\x12\x0cgrpc.testing\"\xdc\x01\n\x0bServer\
    Stats\x12!\n\x0ctime_elapsed\x18\x01\x20\x01(\x01R\x0btimeElapsed\x12\
    \x1b\n\ttime_user\x18\x02\x20\x01(\x01R\x08timeUser\x12\x1f\n\x0btime_sy\
    stem\x18\x03\x20\x01(\x01R\ntimeSystem\x12$\n\x0etotal_cpu_time\x18\x04\
    \x20\x01(\x04R\x0ctotalCpuTime\x12\"\n\ridle_cpu_time\x18\x05\x20\x01(\
    \x04R\x0bidleCpuTime\x12\"\n\rcq_poll_count\x18\x06\x20\x01(\x04R\x0bcqP\
    ollCount\"T\n\x0fHistogramParams\x12\x1e\n\nresolution\x18\x01\x20\x01(\
    \x01R\nresolution\x12!\n\x0cmax_possible\x18\x02\x20\x01(\x01R\x0bmaxPos\
    sible\"\xab\x01\n\rHistogramData\x12\x16\n\x06bucket\x18\x01\x20\x03(\rR\
    \x06bucket\x12\x19\n\x08min_seen\x18\x02\x20\x01(\x01R\x07minSeen\x12\
    \x19\n\x08max_seen\x18\x03\x20\x01(\x01R\x07maxSeen\x12\x10\n\x03sum\x18\
    \x04\x20\x01(\x01R\x03sum\x12$\n\x0esum_of_squares\x18\x05\x20\x01(\x01R\
    \x0csumOfSquares\x12\x14\n\x05count\x18\x06\x20\x01(\x01R\x05count\"K\n\
    \x12RequestResultCount\x12\x1f\n\x0bstatus_code\x18\x01\x20\x01(\x05R\ns\
    tatusCode\x12\x14\n\x05count\x18\x02\x20\x01(\x03R\x05count\"\x98\x02\n\
    \x0bClientStats\x129\n\tlatencies\x18\x01\x20\x01(\x0b2\x1b.grpc.testing\
    .HistogramDataR\tlatencies\x12!\n\x0ctime_elapsed\x18\x02\x20\x01(\x01R\
    \x0btimeElapsed\x12\x1b\n\ttime_user\x18\x03\x20\x01(\x01R\x08timeUser\
    \x12\x1f\n\x0btime_system\x18\x04\x20\x01(\x01R\ntimeSystem\x12I\n\x0fre\
    quest_results\x18\x05\x20\x03(\x0b2\x20.grpc.testing.RequestResultCountR\
    \x0erequestResults\x12\"\n\rcq_poll_count\x18\x06\x20\x01(\x04R\x0bcqPol\
    lCountb\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(5);
            messages.push(ServerStats::generated_message_descriptor_data());
            messages.push(HistogramParams::generated_message_descriptor_data());
            messages.push(HistogramData::generated_message_descriptor_data());
            messages.push(RequestResultCount::generated_message_descriptor_data());
            messages.push(ClientStats::generated_message_descriptor_data());
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
