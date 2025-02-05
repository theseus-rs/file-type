use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3967535666: FileFormat = FileFormat {
    id: 3_967_535_666,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml"],
    signatures: &[],
    related_formats: &[],
};
