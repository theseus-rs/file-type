use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_83369969: FileType = FileType {
    file_format: &FileFormat {
        id: 83_369_969,
        source_type: SourceType::Wikidata,
        name: "FastTracker2 Extended",
        extensions: &["xi", "xm"],
        media_types: &["audio/x-xi", "audio/x-xm"],
        signatures: &[],
        related_formats: &[],
    },
};
