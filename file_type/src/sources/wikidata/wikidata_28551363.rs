use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28551363: FileType = FileType {
    file_format: &FileFormat {
        id: 28_551_363,
        source_type: SourceType::Wikidata,
        name: "Adobe Levels File",
        extensions: &["alv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
