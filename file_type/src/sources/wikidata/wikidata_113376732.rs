use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113376732: FileType = FileType {
    file_format: &FileFormat {
        id: 113_376_732,
        source_type: SourceType::Wikidata,
        name: "Easy CD Creator Layout, version 5-6",
        extensions: &["cl5", "rcl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
