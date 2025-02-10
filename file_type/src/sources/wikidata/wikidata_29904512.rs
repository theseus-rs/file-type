use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29904512: FileType = FileType {
    file_format: &FileFormat {
        id: 29_904_512,
        source_type: SourceType::Wikidata,
        name: "SAR",
        extensions: &["sar"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
