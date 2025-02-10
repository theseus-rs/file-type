use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_76159238: FileType = FileType {
    file_format: &FileFormat {
        id: 76_159_238,
        source_type: SourceType::Wikidata,
        name: "GDAL Raster Virtual Format",
        extensions: &["vrt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
