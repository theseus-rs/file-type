use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1166187818: FileFormat = FileFormat {
    id: 1_166_187_818,
    source_type: SourceType::Iana,
    name: "vnd.nokia.pcd+wbxml",
    extensions: &[],
    media_types: &["application/vnd.nokia.pcd+wbxml"],
    signatures: &[],
    related_formats: &[],
};
