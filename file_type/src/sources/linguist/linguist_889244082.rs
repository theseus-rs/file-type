use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_889244082: FileType = FileType {
    file_format: &FileFormat {
        id: 889_244_082,
        source_type: SourceType::Linguist,
        name: "Odin",
        extensions: &["odin"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
