use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113578632: FileType = FileType {
    file_format: &FileFormat {
        id: 113_578_632,
        source_type: SourceType::Wikidata,
        name: "MAGIX photo album",
        extensions: &["alb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
