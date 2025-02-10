use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_83548846: FileType = FileType {
    file_format: &FileFormat {
        id: 83_548_846,
        source_type: SourceType::Wikidata,
        name: "Nearly Raw Raster Data, version 3",
        extensions: &["nrrd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
