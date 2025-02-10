use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112820809: FileType = FileType {
    file_format: &FileFormat {
        id: 112_820_809,
        source_type: SourceType::Wikidata,
        name: "LightWave binary object file",
        extensions: &["lw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
