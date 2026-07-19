use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138113875: FileType = FileType {
    file_format: &FileFormat {
        id: 138_113_875,
        source_type: SourceType::Wikidata,
        name: "VWH",
        extensions: &["vwh"],
        media_types: &["application/vnd.vwh"],
        signatures: &[],
        related_formats: &[],
    },
};
