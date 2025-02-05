use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1142814227: FileFormat = FileFormat {
    id: 1_142_814_227,
    source_type: SourceType::Iana,
    name: "vnd.crick.clicker.palette",
    extensions: &[],
    media_types: &["application/vnd.crick.clicker.palette"],
    signatures: &[],
    related_formats: &[],
};
