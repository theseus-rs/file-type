use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59210786: FileType = FileType {
    file_format: &FileFormat {
        id: 59_210_786,
        source_type: SourceType::Wikidata,
        name: "BIM Collaboration Format",
        extensions: &["bcf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
