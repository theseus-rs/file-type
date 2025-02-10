use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27863192: FileType = FileType {
    file_format: &FileFormat {
        id: 27_863_192,
        source_type: SourceType::Wikidata,
        name: "Audio Data Interchange Format",
        extensions: &["aac"],
        media_types: &["audio/aac"],
        signatures: &[],
        related_formats: &[],
    },
};
