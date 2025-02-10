use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120867887: FileType = FileType {
    file_format: &FileFormat {
        id: 120_867_887,
        source_type: SourceType::Wikidata,
        name: "Cumulus Category Exchange File",
        extensions: &["cce"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
