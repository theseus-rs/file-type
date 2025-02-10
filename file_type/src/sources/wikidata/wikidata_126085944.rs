use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126085944: FileType = FileType {
    file_format: &FileFormat {
        id: 126_085_944,
        source_type: SourceType::Wikidata,
        name: "IMF Package Asset Map",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
