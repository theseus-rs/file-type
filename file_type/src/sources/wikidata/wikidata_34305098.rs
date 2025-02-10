use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34305098: FileType = FileType {
    file_format: &FileFormat {
        id: 34_305_098,
        source_type: SourceType::Wikidata,
        name: "Sassy Cascading Style Sheets",
        extensions: &["scss"],
        media_types: &["text/x-scss"],
        signatures: &[],
        related_formats: &[],
    },
};
