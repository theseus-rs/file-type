use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4052293624: FileFormat = FileFormat {
    id: 4_052_293_624,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.seal-unicast-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.seal-unicast-info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
