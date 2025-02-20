use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
