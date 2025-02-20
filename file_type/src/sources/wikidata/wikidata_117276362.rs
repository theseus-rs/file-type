use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117276362: FileType = FileType {
    file_format: &FileFormat {
        id: 117_276_362,
        source_type: SourceType::Wikidata,
        name: "Ultimate Business Planner File",
        extensions: &["bp1"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
