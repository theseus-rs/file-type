use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123668527: FileType = FileType {
    file_format: &FileFormat {
        id: 123_668_527,
        source_type: SourceType::Wikidata,
        name: "LiveCode Stack 8.1+",
        extensions: &["livecode", "rev"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
