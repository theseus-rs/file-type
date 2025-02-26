use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_239357863: FileType = FileType {
    file_format: &FileFormat {
        id: 239_357_863,
        source_type: SourceType::Linguist,
        name: "Slang",
        extensions: &["slang"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
