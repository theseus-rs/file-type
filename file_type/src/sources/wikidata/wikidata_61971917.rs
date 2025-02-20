use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61971917: FileType = FileType {
    file_format: &FileFormat {
        id: 61_971_917,
        source_type: SourceType::Wikidata,
        name: "FoxPro Database, version 2x",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
