use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2873474993: FileFormat = FileFormat {
    id: 2_873_474_993,
    source_type: SourceType::Iana,
    name: "vnd.wfa.p2p",
    extensions: &[],
    media_types: &["application/vnd.wfa.p2p"],
    signatures: &[],
    related_formats: &[],
};
