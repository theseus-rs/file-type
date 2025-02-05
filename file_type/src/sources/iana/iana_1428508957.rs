use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1428508957: FileFormat = FileFormat {
    id: 1_428_508_957,
    source_type: SourceType::Iana,
    name: "3gpp2",
    extensions: &[],
    media_types: &["audio/3gpp2"],
    signatures: &[],
    related_formats: &[],
};
