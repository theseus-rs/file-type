use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206455: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_455,
        source_type: SourceType::Wikidata,
        name: "CKiSS",
        extensions: &["cel"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
