use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59999653: FileType = FileType {
    file_format: &FileFormat {
        id: 59_999_653,
        source_type: SourceType::Wikidata,
        name: "ESRI Arc/View Project",
        extensions: &["apr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
