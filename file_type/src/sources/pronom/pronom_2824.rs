use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2824: FileType = FileType {
    file_format: &FileFormat {
        id: 2_824,
        source_type: SourceType::Pronom,
        name: "Melco OFM Project",
        extensions: &["ofm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
