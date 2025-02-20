use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5636096: FileType = FileType {
    file_format: &FileFormat {
        id: 5_636_096,
        source_type: SourceType::Wikidata,
        name: "HTML Components",
        extensions: &["htc"],
        media_types: &["text/x-component"],
        signatures: &[],
        related_formats: &[],
    },
};
