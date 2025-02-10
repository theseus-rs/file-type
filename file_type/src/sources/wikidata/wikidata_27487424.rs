use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27487424: FileType = FileType {
    file_format: &FileFormat {
        id: 27_487_424,
        source_type: SourceType::Wikidata,
        name: "Shapefile projections definitions file",
        extensions: &["prj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
