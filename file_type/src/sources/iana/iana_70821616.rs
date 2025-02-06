use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_70821616: FileFormat = FileFormat {
    id: 70_821_616,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.comments+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml"],
    signatures: &[],
    related_formats: &[],
};
