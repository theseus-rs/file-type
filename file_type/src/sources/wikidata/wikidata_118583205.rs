use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118583205: FileType = FileType {
    file_format: &FileFormat {
        id: 118_583_205,
        source_type: SourceType::Wikidata,
        name: "Project5 Project",
        extensions: &["p5p"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
