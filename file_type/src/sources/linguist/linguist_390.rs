use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_390: FileType = FileType {
    file_format: &FileFormat {
        id: 390,
        source_type: SourceType::Linguist,
        name: "Volt",
        extensions: &["volt"],
        media_types: &["text/x-d"],
        signatures: &[],
        related_formats: &[],
    },
};
