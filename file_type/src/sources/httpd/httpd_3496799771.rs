use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3496799771: FileFormat = FileFormat {
    id: 3_496_799_771,
    source_type: SourceType::Httpd,
    name: "tcpdump pcap",
    extensions: &["pcap", "cap", "dmp"],
    media_types: &["application/vnd.tcpdump.pcap"],
    internal_signatures: &[],
    related_formats: &[],
};
