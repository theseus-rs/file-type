use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205801: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_801,
        source_type: SourceType::Wikidata,
        name: "IMG Picture Format",
        extensions: &["img"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
