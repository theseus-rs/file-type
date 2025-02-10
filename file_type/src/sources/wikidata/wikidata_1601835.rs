use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1601835: FileType = FileType {
    file_format: &FileFormat {
        id: 1_601_835,
        source_type: SourceType::Wikidata,
        name: "Standard Test Data Format",
        extensions: &["std", "stdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
