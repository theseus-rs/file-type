use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_73794214: FileType = FileType {
    file_format: &FileFormat {
        id: 73_794_214,
        source_type: SourceType::Wikidata,
        name: "QGIS Layer",
        extensions: &["qlr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
