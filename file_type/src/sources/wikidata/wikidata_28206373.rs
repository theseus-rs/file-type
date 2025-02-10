use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206373: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_373,
        source_type: SourceType::Wikidata,
        name: "Interleaf image",
        extensions: &["iimg", "img"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
