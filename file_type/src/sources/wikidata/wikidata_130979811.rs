use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130979811: FileType = FileType {
    file_format: &FileFormat {
        id: 130_979_811,
        source_type: SourceType::Wikidata,
        name: "Slim file format",
        extensions: &["slim"],
        media_types: &["text/x-slim"],
        signatures: &[],
        related_formats: &[],
    },
};
