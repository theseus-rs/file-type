use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3011110: FileType = FileType {
    file_format: &FileFormat {
        id: 3_011_110,
        source_type: SourceType::Wikidata,
        name: "DAR",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
