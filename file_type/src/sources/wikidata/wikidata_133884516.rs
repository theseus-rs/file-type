use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133884516: FileType = FileType {
    file_format: &FileFormat {
        id: 133_884_516,
        source_type: SourceType::Wikidata,
        name: "YAFA",
        extensions: &["yafa"],
        media_types: &["video/x-yafa"],
        signatures: &[],
        related_formats: &[],
    },
};
