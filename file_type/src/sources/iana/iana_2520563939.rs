use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2520563939: FileFormat = FileFormat {
    id: 2_520_563_939,
    source_type: SourceType::Iana,
    name: "vnd.gridmp",
    extensions: &[],
    media_types: &["application/vnd.gridmp"],
    signatures: &[],
    related_formats: &[],
};
