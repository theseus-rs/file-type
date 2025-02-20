use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127327939: FileType = FileType {
    file_format: &FileFormat {
        id: 127_327_939,
        source_type: SourceType::Wikidata,
        name: "COBOL Source Code File",
        extensions: &["cbl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
