use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206538: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_538,
        source_type: SourceType::Wikidata,
        name: "Magick Persistent Cache (.cache file)",
        extensions: &["cache"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
