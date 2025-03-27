use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2819247: FileType = FileType {
    file_format: &FileFormat {
        id: 2_819_247,
        source_type: SourceType::Wikidata,
        name: "ALTO",
        extensions: &["xml"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
