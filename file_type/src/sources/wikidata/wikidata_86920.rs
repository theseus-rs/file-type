use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_86920: FileType = FileType {
    file_format: &FileFormat {
        id: 86_920,
        source_type: SourceType::Wikidata,
        name: "text file",
        extensions: &["text", "txt"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
