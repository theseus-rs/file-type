use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207058: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_058,
        source_type: SourceType::Wikidata,
        name: "Poser Bump Map",
        extensions: &["bum"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
