use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_281876: FileType = FileType {
    file_format: &FileFormat {
        id: 281_876,
        source_type: SourceType::Wikidata,
        name: "YAML",
        extensions: &["yaml", "yml"],
        media_types: &["application/yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
