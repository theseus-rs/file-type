use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28600493: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_493,
        source_type: SourceType::Wikidata,
        name: "DeltaVision",
        extensions: &["dv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
