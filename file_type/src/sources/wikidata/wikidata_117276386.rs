use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117276386: FileType = FileType {
    file_format: &FileFormat {
        id: 117_276_386,
        source_type: SourceType::Wikidata,
        name: "Old Business Planner File",
        extensions: &["udf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
