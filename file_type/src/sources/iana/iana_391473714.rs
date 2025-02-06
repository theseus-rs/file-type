use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_391473714: FileFormat = FileFormat {
    id: 391_473_714,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"],
    signatures: &[],
    related_formats: &[],
};
