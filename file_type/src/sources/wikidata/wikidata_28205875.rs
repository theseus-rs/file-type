use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205875: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_875,
        source_type: SourceType::Wikidata,
        name: "CUPS Raster",
        extensions: &[],
        media_types: &["application/vnd.cups-raster"],
        signatures: &[],
        related_formats: &[],
    },
};
