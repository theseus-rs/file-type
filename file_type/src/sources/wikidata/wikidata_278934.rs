use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_278934: FileType = FileType {
    file_format: &FileFormat {
        id: 278_934,
        source_type: SourceType::Wikidata,
        name: "shapefile",
        extensions: &["dbf", "shp", "shx"],
        media_types: &[
            "application/vnd.dbf",
            "application/vnd.shp",
            "application/vnd.shx",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
