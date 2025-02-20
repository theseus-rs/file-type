use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117448429: FileType = FileType {
    file_format: &FileFormat {
        id: 117_448_429,
        source_type: SourceType::Wikidata,
        name: "CHAT Transcription Format",
        extensions: &["cha"],
        media_types: &["text/x-chat"],
        signatures: &[],
        related_formats: &[],
    },
};
