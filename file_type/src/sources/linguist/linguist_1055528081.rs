use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_1055528081: FileType = FileType {
    file_format: &FileFormat {
        id: 1_055_528_081,
        source_type: SourceType::Linguist,
        name: "Bikeshed",
        extensions: &["bs"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
