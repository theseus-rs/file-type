use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131287731: FileType = FileType {
    file_format: &FileFormat {
        id: 131_287_731,
        source_type: SourceType::Wikidata,
        name: "Tea Template file format",
        extensions: &["tea"],
        media_types: &["text/x-tea"],
        signatures: &[],
        related_formats: &[],
    },
};
