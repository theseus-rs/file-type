use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_695876004: FileType = FileType {
    file_format: &FileFormat {
        id: 695_876_004,
        source_type: SourceType::Iana,
        name: "vnd.sss-ntf",
        extensions: &[],
        media_types: &["application/vnd.sss-ntf"],
        signatures: &[],
        related_formats: &[],
    },
};
