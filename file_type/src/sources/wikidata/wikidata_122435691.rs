use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122435691: FileType = FileType {
    file_format: &FileFormat {
        id: 122_435_691,
        source_type: SourceType::Wikidata,
        name: "NovaBACKUP Job",
        extensions: &["nbk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
