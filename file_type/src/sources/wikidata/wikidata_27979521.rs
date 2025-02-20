use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979521: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_521,
        source_type: SourceType::Wikidata,
        name: "Plex Video Preview Thumbnail",
        extensions: &["bif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
