use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112653362: FileType = FileType {
    file_format: &FileFormat {
        id: 112_653_362,
        source_type: SourceType::Wikidata,
        name: "Astound Draw file",
        extensions: &["adw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
