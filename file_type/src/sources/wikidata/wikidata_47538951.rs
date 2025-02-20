use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47538951: FileType = FileType {
    file_format: &FileFormat {
        id: 47_538_951,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Compiled Menu",
        extensions: &["met"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
