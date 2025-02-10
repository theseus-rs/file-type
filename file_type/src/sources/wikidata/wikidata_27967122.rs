use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967122: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_122,
        source_type: SourceType::Wikidata,
        name: "Brian Postma SoundMon v2.x & v3.x module",
        extensions: &["bp3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
