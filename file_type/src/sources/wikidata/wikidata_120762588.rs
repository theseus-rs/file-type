use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120762588: FileType = FileType {
    file_format: &FileFormat {
        id: 120_762_588,
        source_type: SourceType::Wikidata,
        name: "Topo USA File",
        extensions: &["tpx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
