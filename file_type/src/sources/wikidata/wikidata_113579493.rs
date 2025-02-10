use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113579493: FileType = FileType {
    file_format: &FileFormat {
        id: 113_579_493,
        source_type: SourceType::Wikidata,
        name: "Justfile",
        extensions: &["just", "justfile"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
