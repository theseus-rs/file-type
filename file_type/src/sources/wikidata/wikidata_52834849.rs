use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_52834849: FileType = FileType {
    file_format: &FileFormat {
        id: 52_834_849,
        source_type: SourceType::Wikidata,
        name: "dBASE for Windows database, version 5",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
