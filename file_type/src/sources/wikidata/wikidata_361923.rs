use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_361923: FileType = FileType {
    file_format: &FileFormat {
        id: 361_923,
        source_type: SourceType::Wikidata,
        name: "Lossless predictive audio compression",
        extensions: &["pac"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
