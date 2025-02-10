use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_603371597: FileType = FileType {
    file_format: &FileFormat {
        id: 603_371_597,
        source_type: SourceType::Linguist,
        name: "V",
        extensions: &["v"],
        media_types: &["text/x-go"],
        signatures: &[],
        related_formats: &[],
    },
};
