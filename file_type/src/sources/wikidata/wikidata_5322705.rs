use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_5322705: FileType = FileType {
    file_format: &FileFormat {
        id: 5_322_705,
        source_type: SourceType::Wikidata,
        name: "Extended Common Object File Format",
        extensions: &["o", "so"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
