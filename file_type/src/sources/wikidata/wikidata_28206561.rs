use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206561: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_561,
        source_type: SourceType::Wikidata,
        name: "Maya IFF",
        extensions: &["iff", "tdi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
