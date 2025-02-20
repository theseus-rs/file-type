use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
