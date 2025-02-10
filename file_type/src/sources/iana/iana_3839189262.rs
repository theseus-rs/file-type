use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3839189262: FileType = FileType {
    file_format: &FileFormat {
        id: 3_839_189_262,
        source_type: SourceType::Iana,
        name: "vnd.onepagertat",
        extensions: &[],
        media_types: &["application/vnd.onepagertat"],
        signatures: &[],
        related_formats: &[],
    },
};
