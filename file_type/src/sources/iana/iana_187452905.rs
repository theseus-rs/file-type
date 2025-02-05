use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_187452905: FileFormat = FileFormat {
    id: 187_452_905,
    source_type: SourceType::Iana,
    name: "vnd.openeye.oeb",
    extensions: &[],
    media_types: &["application/vnd.openeye.oeb"],
    signatures: &[],
    related_formats: &[],
};
