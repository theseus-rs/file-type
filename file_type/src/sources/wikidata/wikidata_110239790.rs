use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110239790: FileType = FileType {
    file_format: &FileFormat {
        id: 110_239_790,
        source_type: SourceType::Wikidata,
        name: "JData",
        extensions: &["jdb", "jdt"],
        media_types: &["application/jdata-binary", "application/jdata-text"],
        signatures: &[],
        related_formats: &[],
    },
};
