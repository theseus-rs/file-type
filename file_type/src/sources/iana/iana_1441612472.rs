use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1441612472: FileFormat = FileFormat {
    id: 1_441_612_472,
    source_type: SourceType::Iana,
    name: "vnd.eszigno3+xml",
    extensions: &[],
    media_types: &["application/vnd.eszigno3+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
