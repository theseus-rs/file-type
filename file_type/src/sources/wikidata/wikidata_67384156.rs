use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67384156: FileType = FileType {
    file_format: &FileFormat {
        id: 67_384_156,
        source_type: SourceType::Wikidata,
        name: "SimLife Animal",
        extensions: &["anl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
