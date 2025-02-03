use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2303840961: FileFormat = FileFormat {
    id: 2_303_840_961,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.SRVCC-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.SRVCC-info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
