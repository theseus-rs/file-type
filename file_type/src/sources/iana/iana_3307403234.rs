use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3307403234: FileFormat = FileFormat {
    id: 3_307_403_234,
    source_type: SourceType::Iana,
    name: "vnd.Kinar",
    extensions: &[],
    media_types: &["application/vnd.Kinar"],
    signatures: &[],
    related_formats: &[],
};
