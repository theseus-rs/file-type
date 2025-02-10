use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206952: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_952,
        source_type: SourceType::Wikidata,
        name: "PhotoDeluxe",
        extensions: &["pdd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
