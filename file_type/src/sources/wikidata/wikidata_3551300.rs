use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3551300: FileType = FileType {
    file_format: &FileFormat {
        id: 3_551_300,
        source_type: SourceType::Wikidata,
        name: "Universal Subtitle Format",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
