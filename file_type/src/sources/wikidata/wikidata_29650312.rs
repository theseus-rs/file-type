use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29650312: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_312,
        source_type: SourceType::Wikidata,
        name: "PMA",
        extensions: &["pma"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
