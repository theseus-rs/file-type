use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109944567: FileType = FileType {
    file_format: &FileFormat {
        id: 109_944_567,
        source_type: SourceType::Wikidata,
        name: "Generic CADD file format",
        extensions: &["gcd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
