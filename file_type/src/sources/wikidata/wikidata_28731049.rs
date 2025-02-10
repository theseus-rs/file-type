use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28731049: FileType = FileType {
    file_format: &FileFormat {
        id: 28_731_049,
        source_type: SourceType::Wikidata,
        name: "Dyalog APL Transfer File format",
        extensions: &["dxf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
