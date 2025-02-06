use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1821449361: FileFormat = FileFormat {
    id: 1_821_449_361,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml"],
    signatures: &[],
    related_formats: &[],
};
