use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206057: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_057,
        source_type: SourceType::Wikidata,
        name: "ERDAS IMAGINE Gray-scale Bitmap Image",
        extensions: &["gis"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
