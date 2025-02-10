use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29650299: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_299,
        source_type: SourceType::Wikidata,
        name: "PUZ",
        extensions: &["puz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
