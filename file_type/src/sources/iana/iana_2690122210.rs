use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2690122210: FileFormat = FileFormat {
    id: 2_690_122_210,
    source_type: SourceType::Iana,
    name: "vnd.tableschema+json",
    extensions: &[],
    media_types: &["application/vnd.tableschema+json"],
    internal_signatures: &[],
    related_formats: &[],
};
