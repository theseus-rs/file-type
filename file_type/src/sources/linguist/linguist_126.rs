use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_126: FileType = FileType {
    file_format: &FileFormat {
        id: 126,
        source_type: SourceType::Linguist,
        name: "Genshi",
        extensions: &["kid"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
