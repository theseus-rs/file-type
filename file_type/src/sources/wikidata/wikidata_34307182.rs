use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34307182: FileType = FileType {
    file_format: &FileFormat {
        id: 34_307_182,
        source_type: SourceType::Wikidata,
        name: "ScreenWriter II",
        extensions: &["text"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
