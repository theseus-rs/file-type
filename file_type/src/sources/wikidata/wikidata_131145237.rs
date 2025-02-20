use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131145237: FileType = FileType {
    file_format: &FileFormat {
        id: 131_145_237,
        source_type: SourceType::Wikidata,
        name: "SourcePawn source code file",
        extensions: &["sp"],
        media_types: &["text/x-sourcepawn"],
        signatures: &[],
        related_formats: &[],
    },
};
