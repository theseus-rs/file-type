use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_225167241: FileType = FileType {
    file_format: &FileFormat {
        id: 225_167_241,
        source_type: SourceType::Linguist,
        name: "XCompose",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
