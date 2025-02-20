use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1615: FileType = FileType {
    file_format: &FileFormat {
        id: 1_615,
        source_type: SourceType::Pronom,
        name: "StarOffice Impress",
        extensions: &["sdd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
