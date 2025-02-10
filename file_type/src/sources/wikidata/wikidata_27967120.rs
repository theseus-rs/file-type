use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967120: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_120,
        source_type: SourceType::Wikidata,
        name: "Brian Postma SoundMon v1.x module",
        extensions: &["bp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
