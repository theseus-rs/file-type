use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59653966: FileType = FileType {
    file_format: &FileFormat {
        id: 59_653_966,
        source_type: SourceType::Wikidata,
        name: "MySQL Table Definition Format",
        extensions: &["frm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
