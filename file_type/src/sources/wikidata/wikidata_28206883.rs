use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206883: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_883,
        source_type: SourceType::Wikidata,
        name: "Planetary Data System",
        extensions: &["img", "imq", "lbl", "pds"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
