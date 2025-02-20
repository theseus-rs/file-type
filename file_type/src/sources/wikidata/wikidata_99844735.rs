use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_99844735: FileType = FileType {
    file_format: &FileFormat {
        id: 99_844_735,
        source_type: SourceType::Wikidata,
        name: "GDAL Vector Virtual Format",
        extensions: &["vrt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
