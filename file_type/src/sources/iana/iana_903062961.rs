use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_903062961: FileFormat = FileFormat {
    id: 903_062_961,
    source_type: SourceType::Iana,
    name: "vnd.oracle.resource+json",
    extensions: &[],
    media_types: &["application/vnd.oracle.resource+json"],
    internal_signatures: &[],
    related_formats: &[],
};
