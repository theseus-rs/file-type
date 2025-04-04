use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_759: FileType = FileType {
    file_format: &FileFormat {
        id: 759,
        source_type: SourceType::Pronom,
        name: "StarOffice Impress",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
