use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1166187818: FileType = FileType {
    file_format: &FileFormat {
        id: 1_166_187_818,
        source_type: SourceType::Iana,
        name: "vnd.nokia.pcd+wbxml",
        extensions: &[],
        media_types: &["application/vnd.nokia.pcd+wbxml"],
        signatures: &[],
        related_formats: &[],
    },
};
