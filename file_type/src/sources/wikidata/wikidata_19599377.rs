use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_19599377: FileType = FileType {
    file_format: &FileFormat {
        id: 19_599_377,
        source_type: SourceType::Wikidata,
        name: "AppleLink Package Compression Format",
        extensions: &["pkg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
