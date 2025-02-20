use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_49812221: FileType = FileType {
    file_format: &FileFormat {
        id: 49_812_221,
        source_type: SourceType::Wikidata,
        name: "Vectorworks file format, version 2009",
        extensions: &["vwx"],
        media_types: &["application/vnd.vectorworks"],
        signatures: &[],
        related_formats: &[],
    },
};
