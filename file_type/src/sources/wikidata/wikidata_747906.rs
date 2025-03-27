use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_747906: FileType = FileType {
    file_format: &FileFormat {
        id: 747_906,
        source_type: SourceType::Wikidata,
        name: "OpenRaster",
        extensions: &["ora"],
        media_types: &["image/openraster"],
        signatures: &[],
        related_formats: &[],
    },
};
