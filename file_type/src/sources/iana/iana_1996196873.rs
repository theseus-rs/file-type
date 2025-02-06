use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1996196873: FileFormat = FileFormat {
    id: 1_996_196_873,
    source_type: SourceType::Iana,
    name: "bhttp",
    extensions: &[],
    media_types: &["message/bhttp"],
    signatures: &[],
    related_formats: &[],
};
