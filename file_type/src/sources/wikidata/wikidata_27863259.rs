use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27863259: FileType = FileType {
    file_format: &FileFormat {
        id: 27_863_259,
        source_type: SourceType::Wikidata,
        name: "Audio Interchange File Format, version 1.3",
        extensions: &["aif", "aiff"],
        media_types: &["audio/aiff", "audio/x-aiff"],
        signatures: &[],
        related_formats: &[],
    },
};
