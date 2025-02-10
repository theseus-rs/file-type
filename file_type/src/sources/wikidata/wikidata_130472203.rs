use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130472203: FileType = FileType {
    file_format: &FileFormat {
        id: 130_472_203,
        source_type: SourceType::Wikidata,
        name: "Phix file",
        extensions: &["exw"],
        media_types: &["text/x-phix"],
        signatures: &[],
        related_formats: &[],
    },
};
