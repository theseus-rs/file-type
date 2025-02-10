use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_907065713: FileType = FileType {
    file_format: &FileFormat {
        id: 907_065_713,
        source_type: SourceType::Linguist,
        name: "Gemfile.lock",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
