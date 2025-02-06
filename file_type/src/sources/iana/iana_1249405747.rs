use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1249405747: FileFormat = FileFormat {
    id: 1_249_405_747,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"],
    signatures: &[],
    related_formats: &[],
};
