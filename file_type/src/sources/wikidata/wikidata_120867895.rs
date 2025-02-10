use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120867895: FileType = FileType {
    file_format: &FileFormat {
        id: 120_867_895,
        source_type: SourceType::Wikidata,
        name: "Cumulus Record Exchange File",
        extensions: &["cre"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
