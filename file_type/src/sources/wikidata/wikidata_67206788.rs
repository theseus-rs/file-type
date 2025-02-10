use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_67206788: FileType = FileType {
    file_format: &FileFormat {
        id: 67_206_788,
        source_type: SourceType::Wikidata,
        name: "FutureSplash Document",
        extensions: &["spa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
