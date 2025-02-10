use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2370: FileType = FileType {
    file_format: &FileFormat {
        id: 2_370,
        source_type: SourceType::Pronom,
        name: "NeoDesk Icon File",
        extensions: &["nic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
