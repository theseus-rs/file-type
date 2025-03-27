use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34746601: FileType = FileType {
    file_format: &FileFormat {
        id: 34_746_601,
        source_type: SourceType::Wikidata,
        name: "STATISTICA Graph File",
        extensions: &["stg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
