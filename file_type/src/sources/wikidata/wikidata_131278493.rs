use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131278493: FileType = FileType {
    file_format: &FileFormat {
        id: 131_278_493,
        source_type: SourceType::Wikidata,
        name: "Test Anything Protocol output file",
        extensions: &["tap"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
