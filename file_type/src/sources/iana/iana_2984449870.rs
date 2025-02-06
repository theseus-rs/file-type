use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2984449870: FileFormat = FileFormat {
    id: 2_984_449_870,
    source_type: SourceType::Iana,
    name: "vnd.sar",
    extensions: &[],
    media_types: &["application/vnd.sar"],
    signatures: &[],
    related_formats: &[],
};
