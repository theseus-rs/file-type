use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113674320: FileType = FileType {
    file_format: &FileFormat {
        id: 113_674_320,
        source_type: SourceType::Wikidata,
        name: "Final Draft 8 Template",
        extensions: &["fdxt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
