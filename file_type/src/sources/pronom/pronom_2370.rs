use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
