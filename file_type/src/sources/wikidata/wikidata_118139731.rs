use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118139731: FileType = FileType {
    file_format: &FileFormat {
        id: 118_139_731,
        source_type: SourceType::Wikidata,
        name: "Printable Project",
        extensions: &["gwp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
