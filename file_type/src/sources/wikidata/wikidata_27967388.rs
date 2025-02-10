use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967388: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_388,
        source_type: SourceType::Wikidata,
        name: "Adlib Tracker instrument",
        extensions: &["ins"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
