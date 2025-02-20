use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967213: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_213,
        source_type: SourceType::Wikidata,
        name: "Real Tracker module",
        extensions: &["rtm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
