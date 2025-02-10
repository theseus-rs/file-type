use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128628799: FileType = FileType {
    file_format: &FileFormat {
        id: 128_628_799,
        source_type: SourceType::Wikidata,
        name: "BARE schema source",
        extensions: &["bare"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
