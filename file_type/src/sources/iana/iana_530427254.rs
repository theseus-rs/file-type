use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_530427254: FileFormat = FileFormat {
    id: 530_427_254,
    source_type: SourceType::Iana,
    name: "applefile",
    extensions: &[],
    media_types: &["application/applefile"],
    signatures: &[],
    related_formats: &[],
};
