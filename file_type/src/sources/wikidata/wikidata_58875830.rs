use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58875830: FileType = FileType {
    file_format: &FileFormat {
        id: 58_875_830,
        source_type: SourceType::Wikidata,
        name: "PowerProject",
        extensions: &["pp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
