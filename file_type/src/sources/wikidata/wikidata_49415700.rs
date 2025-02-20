use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_49415700: FileType = FileType {
    file_format: &FileFormat {
        id: 49_415_700,
        source_type: SourceType::Wikidata,
        name: "CATIA file format, version 5",
        extensions: &["catmaterial"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
