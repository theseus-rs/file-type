use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27487398: FileType = FileType {
    file_format: &FileFormat {
        id: 27_487_398,
        source_type: SourceType::Wikidata,
        name: "Shapefile geocoding index, ODB format",
        extensions: &["mxs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
