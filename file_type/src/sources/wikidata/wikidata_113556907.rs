use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113556907: FileType = FileType {
    file_format: &FileFormat {
        id: 113_556_907,
        source_type: SourceType::Wikidata,
        name: "Duplicator CD Image File",
        extensions: &["tao"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
