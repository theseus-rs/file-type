use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967203: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_203,
        source_type: SourceType::Wikidata,
        name: "NoiseTracker module",
        extensions: &["mod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
