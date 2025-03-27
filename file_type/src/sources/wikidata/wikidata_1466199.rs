use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1466199: FileType = FileType {
    file_format: &FileFormat {
        id: 1_466_199,
        source_type: SourceType::Wikidata,
        name: "Opus",
        extensions: &["opus"],
        media_types: &["audio/ogg"],
        signatures: &[],
        related_formats: &[],
    },
};
