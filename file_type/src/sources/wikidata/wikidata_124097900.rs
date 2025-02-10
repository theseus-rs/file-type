use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124097900: FileType = FileType {
    file_format: &FileFormat {
        id: 124_097_900,
        source_type: SourceType::Wikidata,
        name: ".txt file",
        extensions: &["txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
