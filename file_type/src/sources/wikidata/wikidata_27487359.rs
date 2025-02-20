use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27487359: FileType = FileType {
    file_format: &FileFormat {
        id: 27_487_359,
        source_type: SourceType::Wikidata,
        name: "ArcView GIS 3.x attribute index",
        extensions: &["atx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
