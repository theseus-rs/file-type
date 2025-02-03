use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_965470757: FileFormat = FileFormat {
    id: 965_470_757,
    source_type: SourceType::Iana,
    name: "vnd.bzip3",
    extensions: &[],
    media_types: &["application/vnd.bzip3"],
    internal_signatures: &[],
    related_formats: &[],
};
