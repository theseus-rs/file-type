use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_479833: FileType = FileType {
    file_format: &FileFormat {
        id: 479_833,
        source_type: SourceType::Wikidata,
        name: "batch file",
        extensions: &["bat", "btm", "cmd", "vbs"],
        media_types: &["application/x-bat"],
        signatures: &[],
        related_formats: &[],
    },
};
