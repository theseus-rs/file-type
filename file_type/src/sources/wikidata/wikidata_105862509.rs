use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105862509: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_509,
        source_type: SourceType::Wikidata,
        name: "Max Patch",
        extensions: &["maxpat"],
        media_types: &["text/json"],
        signatures: &[],
        related_formats: &[],
    },
};
