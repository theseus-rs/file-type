use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138478747: FileType = FileType {
    file_format: &FileFormat {
        id: 138_478_747,
        source_type: SourceType::Wikidata,
        name: "resonitepackage",
        extensions: &["resonitepackage"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
