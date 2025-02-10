use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126087088: FileType = FileType {
    file_format: &FileFormat {
        id: 126_087_088,
        source_type: SourceType::Wikidata,
        name: "IMF Package Composition Playlist",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
