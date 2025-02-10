use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_7866417: FileType = FileType {
    file_format: &FileFormat {
        id: 7_866_417,
        source_type: SourceType::Wikidata,
        name: "USGS DEM",
        extensions: &["dem"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
