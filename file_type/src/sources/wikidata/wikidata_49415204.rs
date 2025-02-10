use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_49415204: FileType = FileType {
    file_format: &FileFormat {
        id: 49_415_204,
        source_type: SourceType::Wikidata,
        name: "CATIA Project, version 4",
        extensions: &["project"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
