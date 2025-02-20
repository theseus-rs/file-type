use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60628005: FileType = FileType {
    file_format: &FileFormat {
        id: 60_628_005,
        source_type: SourceType::Wikidata,
        name: "FoxPro Database, version 2",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
