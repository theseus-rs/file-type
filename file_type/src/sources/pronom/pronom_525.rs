use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_525: FileType = FileType {
    file_format: &FileFormat {
        id: 525,
        source_type: SourceType::Pronom,
        name: "StarOffice Calc",
        extensions: &["sdc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
