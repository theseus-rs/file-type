use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110502382: FileType = FileType {
    file_format: &FileFormat {
        id: 110_502_382,
        source_type: SourceType::Wikidata,
        name: "ISDOC Information System Document (Generic)",
        extensions: &["isdoc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
