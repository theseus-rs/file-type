use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51801746: FileType = FileType {
    file_format: &FileFormat {
        id: 51_801_746,
        source_type: SourceType::Wikidata,
        name: "Stationery for Mac OS X",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
