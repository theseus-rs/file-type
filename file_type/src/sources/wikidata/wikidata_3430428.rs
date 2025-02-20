use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3430428: FileType = FileType {
    file_format: &FileFormat {
        id: 3_430_428,
        source_type: SourceType::Wikidata,
        name: "Rich Text Format Directory",
        extensions: &["rtfd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
