use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1621583018: FileFormat = FileFormat {
    id: 1_621_583_018,
    source_type: SourceType::Iana,
    name: "t140c",
    extensions: &[],
    media_types: &["audio/t140c"],
    internal_signatures: &[],
    related_formats: &[],
};
