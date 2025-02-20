use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59608185: FileType = FileType {
    file_format: &FileFormat {
        id: 59_608_185,
        source_type: SourceType::Wikidata,
        name: "Media View Pro",
        extensions: &["mpcatalog"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
