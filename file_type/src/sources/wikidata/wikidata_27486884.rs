use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27486884: FileType = FileType {
    file_format: &FileFormat {
        id: 27_486_884,
        source_type: SourceType::Wikidata,
        name: "Shapefile main file",
        extensions: &["shp"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
