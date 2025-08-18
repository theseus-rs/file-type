use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135748567: FileType = FileType {
    file_format: &FileFormat {
        id: 135_748_567,
        source_type: SourceType::Wikidata,
        name: "LView Pro file format",
        extensions: &["lvp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
