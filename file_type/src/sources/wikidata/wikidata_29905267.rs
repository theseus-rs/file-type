use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29905267: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_267,
        source_type: SourceType::Wikidata,
        name: "Self-Extracting Archive",
        extensions: &["sea"],
        media_types: &["application/x-sea"],
        signatures: &[],
        related_formats: &[],
    },
};
