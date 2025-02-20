use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_58799992: FileType = FileType {
    file_format: &FileFormat {
        id: 58_799_992,
        source_type: SourceType::Wikidata,
        name: "PowerProject",
        extensions: &["pp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
