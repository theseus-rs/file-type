use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116701684: FileType = FileType {
    file_format: &FileFormat {
        id: 116_701_684,
        source_type: SourceType::Wikidata,
        name: "Mascot Generic Format",
        extensions: &["mgf"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
