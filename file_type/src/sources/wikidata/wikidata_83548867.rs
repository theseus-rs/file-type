use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_83548867: FileType = FileType {
    file_format: &FileFormat {
        id: 83_548_867,
        source_type: SourceType::Wikidata,
        name: "Nearly Raw Raster Data, version 4",
        extensions: &["nrrd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
