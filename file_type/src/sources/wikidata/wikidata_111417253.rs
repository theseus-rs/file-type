use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111417253: FileType = FileType {
    file_format: &FileFormat {
        id: 111_417_253,
        source_type: SourceType::Wikidata,
        name: "Resource Script format",
        extensions: &["rc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
