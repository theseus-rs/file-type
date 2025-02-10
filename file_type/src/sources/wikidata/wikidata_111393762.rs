use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111393762: FileType = FileType {
    file_format: &FileFormat {
        id: 111_393_762,
        source_type: SourceType::Wikidata,
        name: "Database Oasis Template",
        extensions: &["mkt", "mktx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
