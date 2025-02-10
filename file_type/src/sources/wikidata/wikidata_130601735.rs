use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130601735: FileType = FileType {
    file_format: &FileFormat {
        id: 130_601_735,
        source_type: SourceType::Wikidata,
        name: "R console transcript file",
        extensions: &["rout"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
