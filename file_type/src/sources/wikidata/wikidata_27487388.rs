use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27487388: FileType = FileType {
    file_format: &FileFormat {
        id: 27_487_388,
        source_type: SourceType::Wikidata,
        name: "Shapefile geocoding index",
        extensions: &["ixs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
