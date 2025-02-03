use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3485778358: FileFormat = FileFormat {
    id: 3_485_778_358,
    source_type: SourceType::Iana,
    name: "vnd.hp-PCLXL",
    extensions: &[],
    media_types: &["application/vnd.hp-PCLXL"],
    internal_signatures: &[],
    related_formats: &[],
};
