use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3498786448: FileFormat = FileFormat {
    id: 3_498_786_448,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mcdata-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mcdata-info+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
