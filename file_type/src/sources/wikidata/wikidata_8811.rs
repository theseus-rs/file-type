use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_8811: FileType = FileType {
    file_format: &FileFormat {
        id: 8_811,
        source_type: SourceType::Wikidata,
        name: "HTML",
        extensions: &["html"],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
