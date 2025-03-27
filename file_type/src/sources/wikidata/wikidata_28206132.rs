use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206132: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_132,
        source_type: SourceType::Wikidata,
        name: "Self-Running Flexible Line Interpretation",
        extensions: &["srf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
