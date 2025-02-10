use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206638: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_638,
        source_type: SourceType::Wikidata,
        name: "MTV ray tracer bitmap",
        extensions: &["mtv", "pic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
