use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_286: FileType = FileType {
    file_format: &FileFormat {
        id: 286,
        source_type: SourceType::Linguist,
        name: "PigLatin",
        extensions: &["pig"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
