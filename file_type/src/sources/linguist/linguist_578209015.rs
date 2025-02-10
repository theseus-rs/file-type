use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_578209015: FileType = FileType {
    file_format: &FileFormat {
        id: 578_209_015,
        source_type: SourceType::Linguist,
        name: "Astro",
        extensions: &["astro"],
        media_types: &["text/jsx"],
        signatures: &[],
        related_formats: &[],
    },
};
