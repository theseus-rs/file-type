use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122311153: FileType = FileType {
    file_format: &FileFormat {
        id: 122_311_153,
        source_type: SourceType::Wikidata,
        name: "Open Mining Format",
        extensions: &["omf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
