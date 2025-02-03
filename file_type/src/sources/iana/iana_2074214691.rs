use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2074214691: FileFormat = FileFormat {
    id: 2_074_214_691,
    source_type: SourceType::Iana,
    name: "rtf",
    extensions: &[],
    media_types: &["application/rtf"],
    internal_signatures: &[],
    related_formats: &[],
};
