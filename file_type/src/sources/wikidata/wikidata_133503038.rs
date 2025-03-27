use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133503038: FileType = FileType {
    file_format: &FileFormat {
        id: 133_503_038,
        source_type: SourceType::Wikidata,
        name: "Cheese file",
        extensions: &["che"],
        media_types: &["image/x-cheese"],
        signatures: &[],
        related_formats: &[],
    },
};
