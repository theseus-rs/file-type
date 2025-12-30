use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136095526: FileType = FileType {
    file_format: &FileFormat {
        id: 136_095_526,
        source_type: SourceType::Wikidata,
        name: "Audio-only MPEG-4",
        extensions: &["m4a", "m4b"],
        media_types: &["audio/mp4"],
        signatures: &[],
        related_formats: &[],
    },
};
