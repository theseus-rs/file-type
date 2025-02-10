use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_988020015: FileType = FileType {
    file_format: &FileFormat {
        id: 988_020_015,
        source_type: SourceType::Linguist,
        name: "Texinfo",
        extensions: &["texi", "texinfo", "txi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
