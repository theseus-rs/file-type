use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131395429: FileType = FileType {
    file_format: &FileFormat {
        id: 131_395_429,
        source_type: SourceType::Wikidata,
        name: "Verifpal code",
        extensions: &["vp"],
        media_types: &["text/x-verifpal"],
        signatures: &[],
        related_formats: &[],
    },
};
