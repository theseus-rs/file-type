use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28234649: FileType = FileType {
    file_format: &FileFormat {
        id: 28_234_649,
        source_type: SourceType::Wikidata,
        name: "CCITT Group 3",
        extensions: &["g3"],
        media_types: &["image/g3fax"],
        signatures: &[],
        related_formats: &[],
    },
};
