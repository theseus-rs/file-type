use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117424649: FileType = FileType {
    file_format: &FileFormat {
        id: 117_424_649,
        source_type: SourceType::Wikidata,
        name: "Stationery file",
        extensions: &["sta"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
