use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_279254: FileType = FileType {
    file_format: &FileFormat {
        id: 279_254,
        source_type: SourceType::Wikidata,
        name: "a.out",
        extensions: &["o", "so"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
