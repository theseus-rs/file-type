use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111341669: FileType = FileType {
    file_format: &FileFormat {
        id: 111_341_669,
        source_type: SourceType::Wikidata,
        name: "Creative Labs FM instrument",
        extensions: &["sbi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
