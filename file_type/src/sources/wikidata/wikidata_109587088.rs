use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109587088: FileType = FileType {
    file_format: &FileFormat {
        id: 109_587_088,
        source_type: SourceType::Wikidata,
        name: "EasyPhoto Gallery",
        extensions: &["gal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
