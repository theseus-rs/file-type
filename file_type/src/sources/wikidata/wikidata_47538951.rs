use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
