use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_71178742: FileType = FileType {
    file_format: &FileFormat {
        id: 71_178_742,
        source_type: SourceType::Wikidata,
        name: "PIPE2 file format",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
