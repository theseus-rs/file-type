use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_286: FileType = FileType {
    file_format: &FileFormat {
        id: 286,
        source_type: SourceType::Linguist,
        name: "PigLatin",
        extensions: &["pig"],
        media_types: &["text/x-pig"],
        signatures: &[],
        related_formats: &[],
    },
};
