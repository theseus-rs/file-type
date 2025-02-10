use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122074126: FileType = FileType {
    file_format: &FileFormat {
        id: 122_074_126,
        source_type: SourceType::Wikidata,
        name: "Score file",
        extensions: &["pge"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
