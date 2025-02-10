use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123204255: FileType = FileType {
    file_format: &FileFormat {
        id: 123_204_255,
        source_type: SourceType::Wikidata,
        name: "Avid Media Composer Script",
        extensions: &["avc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
