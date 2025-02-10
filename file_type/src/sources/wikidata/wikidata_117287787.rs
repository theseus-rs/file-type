use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117287787: FileType = FileType {
    file_format: &FileFormat {
        id: 117_287_787,
        source_type: SourceType::Wikidata,
        name: "SigmaPlot Regression Library file",
        extensions: &["jfl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
