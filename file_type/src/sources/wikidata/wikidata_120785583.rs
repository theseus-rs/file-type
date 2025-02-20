use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_120785583: FileType = FileType {
    file_format: &FileFormat {
        id: 120_785_583,
        source_type: SourceType::Wikidata,
        name: "BusinessCards format",
        extensions: &["biz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
