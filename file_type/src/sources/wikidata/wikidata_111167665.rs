use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111167665: FileType = FileType {
    file_format: &FileFormat {
        id: 111_167_665,
        source_type: SourceType::Wikidata,
        name: "ChemSketch 1.0 file",
        extensions: &["mst", "rpt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
