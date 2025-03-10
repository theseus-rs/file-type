use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133252316: FileType = FileType {
    file_format: &FileFormat {
        id: 133_252_316,
        source_type: SourceType::Wikidata,
        name: "Open Packaging Format 3",
        extensions: &["opf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
