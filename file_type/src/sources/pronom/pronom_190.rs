use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_190: FileType = FileType {
    file_format: &FileFormat {
        id: 190,
        source_type: SourceType::Pronom,
        name: "Stationery for Mac OS X",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
