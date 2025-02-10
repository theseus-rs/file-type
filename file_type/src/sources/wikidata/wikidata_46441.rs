use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_46441: FileType = FileType {
    file_format: &FileFormat {
        id: 46_441,
        source_type: SourceType::Wikidata,
        name: "Cascading Style Sheets",
        extensions: &["css"],
        media_types: &["text/css"],
        signatures: &[],
        related_formats: &[],
    },
};
