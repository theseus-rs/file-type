use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_124097900: FileType = FileType {
    file_format: &FileFormat {
        id: 124_097_900,
        source_type: SourceType::Wikidata,
        name: ".txt file",
        extensions: &["txt"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
