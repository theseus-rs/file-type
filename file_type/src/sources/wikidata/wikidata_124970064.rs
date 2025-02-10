use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124970064: FileType = FileType {
    file_format: &FileFormat {
        id: 124_970_064,
        source_type: SourceType::Wikidata,
        name: "MIX index file",
        extensions: &["mixindex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
