use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14775850855049637898: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "tcpdump pcap",
    extensions: &["pcap", "cap", "dmp"],
    media_types: &["application/vnd.tcpdump.pcap"],
    internal_signatures: &[],
    related_formats: &[],
};
