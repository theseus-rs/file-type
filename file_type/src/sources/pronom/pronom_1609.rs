use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1609: FileType = FileType {
    file_format: &FileFormat {
        id: 1_609,
        source_type: SourceType::Pronom,
        name: "StarOffice Calc",
        extensions: &["sdc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
