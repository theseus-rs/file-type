use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206498: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_498,
        source_type: SourceType::Wikidata,
        name: "Age of Empires Graphics File",
        extensions: &["slp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
