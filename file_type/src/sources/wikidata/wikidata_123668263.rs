use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123668263: FileType = FileType {
    file_format: &FileFormat {
        id: 123_668_263,
        source_type: SourceType::Wikidata,
        name: "LiveCode Stack 7.0",
        extensions: &["livecode", "rev"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
