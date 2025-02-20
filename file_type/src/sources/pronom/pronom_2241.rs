use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2241: FileType = FileType {
    file_format: &FileFormat {
        id: 2_241,
        source_type: SourceType::Pronom,
        name: "HP TRIM Outlook Saved Message File",
        extensions: &["vmbx", "mbx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
