use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3217181: FileType = FileType {
    file_format: &FileFormat {
        id: 3_217_181,
        source_type: SourceType::Wikidata,
        name: "Emotion Markup Language",
        extensions: &[],
        media_types: &["application/emotionml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
