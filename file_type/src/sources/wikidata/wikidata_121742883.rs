use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_121742883: FileType = FileType {
    file_format: &FileFormat {
        id: 121_742_883,
        source_type: SourceType::Wikidata,
        name: "The Master Genealogist Project",
        extensions: &["pjc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
