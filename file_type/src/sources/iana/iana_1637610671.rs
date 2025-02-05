use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1637610671: FileFormat = FileFormat {
    id: 1_637_610_671,
    source_type: SourceType::Iana,
    name: "MPA",
    extensions: &[],
    media_types: &["audio/MPA"],
    signatures: &[],
    related_formats: &[],
};
