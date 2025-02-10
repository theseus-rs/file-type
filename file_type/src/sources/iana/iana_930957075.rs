use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_930957075: FileType = FileType {
    file_format: &FileFormat {
        id: 930_957_075,
        source_type: SourceType::Iana,
        name: "vnd.mynfc",
        extensions: &[],
        media_types: &["application/vnd.mynfc"],
        signatures: &[],
        related_formats: &[],
    },
};
