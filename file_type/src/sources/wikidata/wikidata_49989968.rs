use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_49989968: FileType = FileType {
    file_format: &FileFormat {
        id: 49_989_968,
        source_type: SourceType::Wikidata,
        name: "Internet Explorer for Mac cache file",
        extensions: &["waf"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
