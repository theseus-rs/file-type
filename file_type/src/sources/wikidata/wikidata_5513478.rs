use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_5513478: FileType = FileType {
    file_format: &FileFormat {
        id: 5_513_478,
        source_type: SourceType::Wikidata,
        name: "GIFT",
        extensions: &["gift"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
