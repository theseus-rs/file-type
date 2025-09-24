use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_906627898: FileType = FileType {
    file_format: &FileFormat {
        id: 906_627_898,
        source_type: SourceType::Linguist,
        name: "Bru",
        extensions: &["bru"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
