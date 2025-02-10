use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206262: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_262,
        source_type: SourceType::Wikidata,
        name: "HSI JPEG",
        extensions: &["hsi", "jpg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
