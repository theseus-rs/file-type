use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_58876023: FileType = FileType {
    file_format: &FileFormat {
        id: 58_876_023,
        source_type: SourceType::Wikidata,
        name: "PowerProject",
        extensions: &["pp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
