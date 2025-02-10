use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112653466: FileType = FileType {
    file_format: &FileFormat {
        id: 112_653_466,
        source_type: SourceType::Wikidata,
        name: "Professional Draw 1.0 file",
        extensions: &["pdw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
