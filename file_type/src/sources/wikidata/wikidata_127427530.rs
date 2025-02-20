use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127427530: FileType = FileType {
    file_format: &FileFormat {
        id: 127_427_530,
        source_type: SourceType::Wikidata,
        name: "GGUF",
        extensions: &["gguf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
