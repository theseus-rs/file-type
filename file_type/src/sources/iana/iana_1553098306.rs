use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1553098306: FileFormat = FileFormat {
    id: 1_553_098_306,
    source_type: SourceType::Iana,
    name: "vnd.leap+json",
    extensions: &[],
    media_types: &["application/vnd.leap+json"],
    signatures: &[],
    related_formats: &[],
};
