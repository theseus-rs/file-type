use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967399: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_399,
        source_type: SourceType::Wikidata,
        name: "AMusic module",
        extensions: &["amd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
