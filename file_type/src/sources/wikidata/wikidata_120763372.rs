use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120763372: FileType = FileType {
    file_format: &FileFormat {
        id: 120_763_372,
        source_type: SourceType::Wikidata,
        name: "Topo USA 3.0 File",
        extensions: &["tp3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
