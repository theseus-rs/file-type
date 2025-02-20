use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_32096599: FileType = FileType {
    file_format: &FileFormat {
        id: 32_096_599,
        source_type: SourceType::Wikidata,
        name: "Gran Turismo File System",
        extensions: &["vol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
