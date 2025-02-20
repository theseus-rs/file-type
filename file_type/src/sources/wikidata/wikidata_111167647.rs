use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111167647: FileType = FileType {
    file_format: &FileFormat {
        id: 111_167_647,
        source_type: SourceType::Wikidata,
        name: "ISIS/Sketch file",
        extensions: &["skc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
