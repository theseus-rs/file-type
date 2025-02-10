use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27863260: FileType = FileType {
    file_format: &FileFormat {
        id: 27_863_260,
        source_type: SourceType::Wikidata,
        name: "Audio Interchange File Format, compressed",
        extensions: &["aifc"],
        media_types: &["audio/aiff", "audio/x-aiff"],
        signatures: &[],
        related_formats: &[],
    },
};
