use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
