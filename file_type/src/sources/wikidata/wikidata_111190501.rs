use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111190501: FileType = FileType {
    file_format: &FileFormat {
        id: 111_190_501,
        source_type: SourceType::Wikidata,
        name: "Visual Tool Markup Language Document",
        extensions: &["vtm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
