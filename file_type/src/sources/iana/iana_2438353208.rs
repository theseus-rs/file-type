use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2438353208: FileFormat = FileFormat {
    id: 2_438_353_208,
    source_type: SourceType::Iana,
    name: "vnd.bint.med-plus",
    extensions: &[],
    media_types: &["multipart/vnd.bint.med-plus"],
    signatures: &[],
    related_formats: &[],
};
