use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
