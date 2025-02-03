use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3101228311: FileFormat = FileFormat {
    id: 3_101_228_311,
    source_type: SourceType::Iana,
    name: "lost+xml",
    extensions: &[],
    media_types: &["application/lost+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
