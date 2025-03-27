use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105592682: FileType = FileType {
    file_format: &FileFormat {
        id: 105_592_682,
        source_type: SourceType::Wikidata,
        name: "Power Tab",
        extensions: &["ptb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
