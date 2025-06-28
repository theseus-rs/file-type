use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_648759486: FileType = FileType {
    file_format: &FileFormat {
        id: 648_759_486,
        source_type: SourceType::Linguist,
        name: "Daslang",
        extensions: &["das"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
