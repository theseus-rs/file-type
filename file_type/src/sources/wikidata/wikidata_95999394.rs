use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_95999394: FileType = FileType {
    file_format: &FileFormat {
        id: 95_999_394,
        source_type: SourceType::Wikidata,
        name: "Formatted Checkpoint file",
        extensions: &["fchk"],
        media_types: &["chemical/x-gaussian-checkpoint"],
        signatures: &[],
        related_formats: &[],
    },
};
