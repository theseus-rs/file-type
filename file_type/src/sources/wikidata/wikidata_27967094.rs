use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967094: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_094,
        source_type: SourceType::Wikidata,
        name: "Interplay ACM",
        extensions: &["acm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
