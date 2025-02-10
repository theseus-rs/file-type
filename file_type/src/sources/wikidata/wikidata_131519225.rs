use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131519225: FileType = FileType {
    file_format: &FileFormat {
        id: 131_519_225,
        source_type: SourceType::Wikidata,
        name: "Stimulate Signal Data",
        extensions: &["sdt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
