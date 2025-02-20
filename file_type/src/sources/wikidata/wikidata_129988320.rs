use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129988320: FileType = FileType {
    file_format: &FileFormat {
        id: 129_988_320,
        source_type: SourceType::Wikidata,
        name: "JMESPath query file",
        extensions: &["jp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
