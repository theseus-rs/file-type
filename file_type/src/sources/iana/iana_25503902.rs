use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_25503902: FileFormat = FileFormat {
    id: 25_503_902,
    source_type: SourceType::Iana,
    name: "related",
    extensions: &[],
    media_types: &["multipart/related"],
    signatures: &[],
    related_formats: &[],
};
