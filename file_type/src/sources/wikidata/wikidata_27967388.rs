use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
