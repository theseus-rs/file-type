use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_49800136: FileType = FileType {
    file_format: &FileFormat {
        id: 49_800_136,
        source_type: SourceType::Wikidata,
        name: "Vectorworks file format, version 12.5",
        extensions: &["vwx"],
        media_types: &["application/vnd.vectorworks"],
        signatures: &[],
        related_formats: &[],
    },
};
