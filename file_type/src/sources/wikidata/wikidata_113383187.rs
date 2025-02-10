use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113383187: FileType = FileType {
    file_format: &FileFormat {
        id: 113_383_187,
        source_type: SourceType::Wikidata,
        name: "Roxio Easy Media Creator Layout 8-10",
        extensions: &["roxio"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
