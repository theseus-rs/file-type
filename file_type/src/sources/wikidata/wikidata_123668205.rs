use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123668205: FileType = FileType {
    file_format: &FileFormat {
        id: 123_668_205,
        source_type: SourceType::Wikidata,
        name: "LiveCode Stack v5.5",
        extensions: &["livecode", "rev"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
