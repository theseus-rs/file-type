use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123385688: FileType = FileType {
    file_format: &FileFormat {
        id: 123_385_688,
        source_type: SourceType::Wikidata,
        name: "iSpace 1.0 Scene file",
        extensions: &["iss"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
