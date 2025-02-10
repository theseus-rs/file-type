use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28344713: FileType = FileType {
    file_format: &FileFormat {
        id: 28_344_713,
        source_type: SourceType::Wikidata,
        name: "Package File",
        extensions: &["pkg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
