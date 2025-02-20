use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757652: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_652,
        source_type: SourceType::Wikidata,
        name: "G64",
        extensions: &["g64"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
