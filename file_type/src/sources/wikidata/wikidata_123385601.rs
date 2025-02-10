use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123385601: FileType = FileType {
    file_format: &FileFormat {
        id: 123_385_601,
        source_type: SourceType::Wikidata,
        name: "Sceneeffect library file",
        extensions: &["sel"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
