use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_82065829: FileType = FileType {
    file_format: &FileFormat {
        id: 82_065_829,
        source_type: SourceType::Wikidata,
        name: "ChiWriter high resolution screen font",
        extensions: &["eft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
