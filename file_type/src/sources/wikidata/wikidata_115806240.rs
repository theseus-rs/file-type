use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_115806240: FileType = FileType {
    file_format: &FileFormat {
        id: 115_806_240,
        source_type: SourceType::Wikidata,
        name: "JSON5",
        extensions: &[],
        media_types: &["application/json5"],
        signatures: &[],
        related_formats: &[],
    },
};
