use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111520019: FileType = FileType {
    file_format: &FileFormat {
        id: 111_520_019,
        source_type: SourceType::Wikidata,
        name: "R program file",
        extensions: &["r"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
