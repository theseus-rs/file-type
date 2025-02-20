use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975884: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_884,
        source_type: SourceType::Wikidata,
        name: "Geometry Data File",
        extensions: &["gdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
