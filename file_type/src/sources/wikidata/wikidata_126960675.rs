use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126960675: FileType = FileType {
    file_format: &FileFormat {
        id: 126_960_675,
        source_type: SourceType::Wikidata,
        name: "VAPI file",
        extensions: &["vapi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
