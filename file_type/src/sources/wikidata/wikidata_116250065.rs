use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116250065: FileType = FileType {
    file_format: &FileFormat {
        id: 116_250_065,
        source_type: SourceType::Wikidata,
        name: "Norton System Doctor configuration file",
        extensions: &["nsd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
