use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28919166: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_166,
        source_type: SourceType::Wikidata,
        name: "GHS Geometry",
        extensions: &["gf", "gft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
