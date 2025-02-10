use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116417701: FileType = FileType {
    file_format: &FileFormat {
        id: 116_417_701,
        source_type: SourceType::Wikidata,
        name: "Design and Print file",
        extensions: &["bro"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
