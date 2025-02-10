use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206497: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_497,
        source_type: SourceType::Wikidata,
        name: "Lossless JPEG",
        extensions: &["jpg", "ljpeg", "ljpg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
