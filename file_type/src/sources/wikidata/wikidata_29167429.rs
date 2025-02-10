use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29167429: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_429,
        source_type: SourceType::Wikidata,
        name: "NovaMind",
        extensions: &["nmind"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
