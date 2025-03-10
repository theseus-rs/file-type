use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123050894: FileType = FileType {
    file_format: &FileFormat {
        id: 123_050_894,
        source_type: SourceType::Wikidata,
        name: "ZLIB compressed data",
        extensions: &[],
        media_types: &["application/zlib"],
        signatures: &[],
        related_formats: &[],
    },
};
