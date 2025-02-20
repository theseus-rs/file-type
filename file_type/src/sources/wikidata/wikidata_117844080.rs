use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117844080: FileType = FileType {
    file_format: &FileFormat {
        id: 117_844_080,
        source_type: SourceType::Wikidata,
        name: "JetFax file",
        extensions: &["jet"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
