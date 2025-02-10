use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114978471: FileType = FileType {
    file_format: &FileFormat {
        id: 114_978_471,
        source_type: SourceType::Wikidata,
        name: "Hollywood Screenwriter Script File",
        extensions: &["hws"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
