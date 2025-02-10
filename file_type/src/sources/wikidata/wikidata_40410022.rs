use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_40410022: FileType = FileType {
    file_format: &FileFormat {
        id: 40_410_022,
        source_type: SourceType::Wikidata,
        name: "Feather",
        extensions: &["feather"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
