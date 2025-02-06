use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1294551089: FileFormat = FileFormat {
    id: 1_294_551_089,
    source_type: SourceType::Iana,
    name: "EVRC1",
    extensions: &[],
    media_types: &["audio/EVRC1"],
    signatures: &[],
    related_formats: &[],
};
