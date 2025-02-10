use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126951583: FileType = FileType {
    file_format: &FileFormat {
        id: 126_951_583,
        source_type: SourceType::Wikidata,
        name: "Kotlin Source Code File",
        extensions: &["kt", "kts"],
        media_types: &["text/x-kotlin"],
        signatures: &[],
        related_formats: &[],
    },
};
