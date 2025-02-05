use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3496799771: FileFormat = FileFormat {
    id: 3_496_799_771,
    source_type: SourceType::Iana,
    name: "vnd.tcpdump.pcap",
    extensions: &[],
    media_types: &["application/vnd.tcpdump.pcap"],
    signatures: &[],
    related_formats: &[],
};
