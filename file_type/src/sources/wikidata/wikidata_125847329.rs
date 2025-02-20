use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125847329: FileType = FileType {
    file_format: &FileFormat {
        id: 125_847_329,
        source_type: SourceType::Wikidata,
        name: "D source code file",
        extensions: &["D"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
