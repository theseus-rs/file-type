use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2276274: FileType = FileType {
    file_format: &FileFormat {
        id: 2_276_274,
        source_type: SourceType::Wikidata,
        name: "System.map",
        extensions: &["map"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
