use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
