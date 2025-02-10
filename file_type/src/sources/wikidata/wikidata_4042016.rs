use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_4042016: FileType = FileType {
    file_format: &FileFormat {
        id: 4_042_016,
        source_type: SourceType::Wikidata,
        name: "KSS",
        extensions: &["kss"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
