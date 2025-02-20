use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967117: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_117,
        source_type: SourceType::Wikidata,
        name: "B's Pro Tracker module",
        extensions: &["bpm", "bps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
