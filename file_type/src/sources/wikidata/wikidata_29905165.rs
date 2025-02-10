use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29905165: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_165,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System backup file",
        extensions: &["sas7bbak", "sb7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
