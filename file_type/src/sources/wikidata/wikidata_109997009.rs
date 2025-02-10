use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109997009: FileType = FileType {
    file_format: &FileFormat {
        id: 109_997_009,
        source_type: SourceType::Wikidata,
        name: "OrgPlus 4 Template",
        extensions: &["ops"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
