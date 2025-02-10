use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111333324: FileType = FileType {
    file_format: &FileFormat {
        id: 111_333_324,
        source_type: SourceType::Wikidata,
        name: "Turtle Beach Pinnacle sound bank",
        extensions: &["psb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
