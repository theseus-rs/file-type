use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120762745: FileType = FileType {
    file_format: &FileFormat {
        id: 120_762_745,
        source_type: SourceType::Wikidata,
        name: "Topo USA 4.0 File",
        extensions: &["tp4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
