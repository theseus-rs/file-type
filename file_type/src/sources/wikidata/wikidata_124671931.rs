use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124671931: FileType = FileType {
    file_format: &FileFormat {
        id: 124_671_931,
        source_type: SourceType::Wikidata,
        name: "Timed Text Markup Language Version 2",
        extensions: &["dfxp", "ttml", "xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
