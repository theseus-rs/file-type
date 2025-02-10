use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_65533032: FileType = FileType {
    file_format: &FileFormat {
        id: 65_533_032,
        source_type: SourceType::Wikidata,
        name: "Menu file format",
        extensions: &["mnu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
