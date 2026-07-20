use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137711100: FileType = FileType {
    file_format: &FileFormat {
        id: 137_711_100,
        source_type: SourceType::Wikidata,
        name: "Qt Phrase Book file",
        extensions: &["qph"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
