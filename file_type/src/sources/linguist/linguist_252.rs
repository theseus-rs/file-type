use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_252: FileType = FileType {
    file_format: &FileFormat {
        id: 252,
        source_type: SourceType::Linguist,
        name: "Nix",
        extensions: &["nix"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
