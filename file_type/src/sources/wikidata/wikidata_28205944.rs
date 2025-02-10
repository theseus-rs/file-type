use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205944: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_944,
        source_type: SourceType::Wikidata,
        name: "Doré Raster",
        extensions: &["dore", "img"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
