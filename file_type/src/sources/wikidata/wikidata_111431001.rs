use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111431001: FileType = FileType {
    file_format: &FileFormat {
        id: 111_431_001,
        source_type: SourceType::Wikidata,
        name: "ExtendScript Included Script File",
        extensions: &["jsxinc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
