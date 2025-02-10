use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2767319262: FileType = FileType {
    file_format: &FileFormat {
        id: 2_767_319_262,
        source_type: SourceType::Iana,
        name: "vnd.ms-asf",
        extensions: &[],
        media_types: &["application/vnd.ms-asf"],
        signatures: &[],
        related_formats: &[],
    },
};
