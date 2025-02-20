use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116869420: FileType = FileType {
    file_format: &FileFormat {
        id: 116_869_420,
        source_type: SourceType::Wikidata,
        name: "Broderbund Print Meta Object",
        extensions: &["pmo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
