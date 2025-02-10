use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_928734530: FileType = FileType {
    file_format: &FileFormat {
        id: 928_734_530,
        source_type: SourceType::Linguist,
        name: "Svelte",
        extensions: &["svelte"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
