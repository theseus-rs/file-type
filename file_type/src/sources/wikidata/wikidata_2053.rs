use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2053: FileType = FileType {
    file_format: &FileFormat {
        id: 2_053,
        source_type: SourceType::Wikidata,
        name: "HTML5",
        extensions: &["htm", "html"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
