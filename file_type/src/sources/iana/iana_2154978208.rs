use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2154978208: FileFormat = FileFormat {
    id: 2_154_978_208,
    source_type: SourceType::Iana,
    name: "vnd.scribus",
    extensions: &[],
    media_types: &["application/vnd.scribus"],
    signatures: &[],
    related_formats: &[],
};
