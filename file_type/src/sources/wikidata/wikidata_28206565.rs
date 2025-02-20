use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206565: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_565,
        source_type: SourceType::Wikidata,
        name: "MicroDesign Area",
        extensions: &["mda"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
