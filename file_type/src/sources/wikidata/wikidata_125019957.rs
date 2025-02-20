use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125019957: FileType = FileType {
    file_format: &FileFormat {
        id: 125_019_957,
        source_type: SourceType::Wikidata,
        name: "GrandView Outline file",
        extensions: &["gv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
