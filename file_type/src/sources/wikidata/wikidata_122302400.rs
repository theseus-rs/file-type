use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122302400: FileType = FileType {
    file_format: &FileFormat {
        id: 122_302_400,
        source_type: SourceType::Wikidata,
        name: "HLD File",
        extensions: &["hld"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
