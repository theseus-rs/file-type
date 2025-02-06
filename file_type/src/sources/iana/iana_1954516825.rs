use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1954516825: FileFormat = FileFormat {
    id: 1_954_516_825,
    source_type: SourceType::Iana,
    name: "EVRCNW0",
    extensions: &[],
    media_types: &["audio/EVRCNW0"],
    signatures: &[],
    related_formats: &[],
};
