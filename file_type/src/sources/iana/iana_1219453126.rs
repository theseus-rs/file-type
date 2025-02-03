use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1219453126: FileFormat = FileFormat {
    id: 1_219_453_126,
    source_type: SourceType::Iana,
    name: "vnd.dart",
    extensions: &[],
    media_types: &["application/vnd.dart"],
    internal_signatures: &[],
    related_formats: &[],
};
