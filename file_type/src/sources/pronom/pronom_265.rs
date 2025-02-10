use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_265: FileType = FileType {
    file_format: &FileFormat {
        id: 265,
        source_type: SourceType::Pronom,
        name: "SAS for MS-DOS Catalog",
        extensions: &["sct"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
