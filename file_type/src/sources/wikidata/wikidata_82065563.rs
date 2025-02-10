use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_82065563: FileType = FileType {
    file_format: &FileFormat {
        id: 82_065_563,
        source_type: SourceType::Wikidata,
        name: "Euphoria Database System",
        extensions: &["edb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
