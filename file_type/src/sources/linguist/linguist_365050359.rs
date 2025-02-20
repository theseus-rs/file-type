use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_365050359: FileType = FileType {
    file_format: &FileFormat {
        id: 365_050_359,
        source_type: SourceType::Linguist,
        name: "Luau",
        extensions: &["luau"],
        media_types: &["text/x-lua"],
        signatures: &[],
        related_formats: &[],
    },
};
