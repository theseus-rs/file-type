use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111333309: FileType = FileType {
    file_format: &FileFormat {
        id: 111_333_309,
        source_type: SourceType::Wikidata,
        name: "Turtle Beach Pinnacle program file",
        extensions: &["ppf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
