use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131695920: FileType = FileType {
    file_format: &FileFormat {
        id: 131_695_920,
        source_type: SourceType::Wikidata,
        name: "Chaco graph partitioning output file",
        extensions: &["coords", "graph"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
