use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120763430: FileType = FileType {
    file_format: &FileFormat {
        id: 120_763_430,
        source_type: SourceType::Wikidata,
        name: "Topo USA 2.0 File",
        extensions: &["tp2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
