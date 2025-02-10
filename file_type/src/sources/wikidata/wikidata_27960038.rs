use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27960038: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_038,
        source_type: SourceType::Wikidata,
        name: "Windows Media Audio Lossless",
        extensions: &["wma", "wmal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
