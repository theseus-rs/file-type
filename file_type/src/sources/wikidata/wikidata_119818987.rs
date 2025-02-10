use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119818987: FileType = FileType {
    file_format: &FileFormat {
        id: 119_818_987,
        source_type: SourceType::Wikidata,
        name: "Final Draft AV Document",
        extensions: &["xav"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
