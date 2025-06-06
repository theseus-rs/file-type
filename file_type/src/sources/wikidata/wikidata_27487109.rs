use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27487109: FileType = FileType {
    file_format: &FileFormat {
        id: 27_487_109,
        source_type: SourceType::Wikidata,
        name: "Shapefile index file",
        extensions: &["shx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
