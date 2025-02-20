use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116869097: FileType = FileType {
    file_format: &FileFormat {
        id: 116_869_097,
        source_type: SourceType::Wikidata,
        name: "Summitsoft Envelope",
        extensions: &["env"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
