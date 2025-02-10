use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109643040: FileType = FileType {
    file_format: &FileFormat {
        id: 109_643_040,
        source_type: SourceType::Wikidata,
        name: "VJ file format",
        extensions: &["vj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
