use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967096: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_096,
        source_type: SourceType::Wikidata,
        name: "Ken's Adlib Music",
        extensions: &["ksm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
