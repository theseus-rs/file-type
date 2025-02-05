use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_316075578: FileFormat = FileFormat {
    id: 316_075_578,
    source_type: SourceType::Iana,
    name: "vnd.dbf",
    extensions: &[],
    media_types: &["application/vnd.dbf"],
    signatures: &[],
    related_formats: &[],
};
