use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126817617: FileType = FileType {
    file_format: &FileFormat {
        id: 126_817_617,
        source_type: SourceType::Wikidata,
        name: "Eiffel Source Code File",
        extensions: &["e"],
        media_types: &["text/x-eiffel"],
        signatures: &[],
        related_formats: &[],
    },
};
