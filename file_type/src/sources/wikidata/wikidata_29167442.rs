use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29167442: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_442,
        source_type: SourceType::Wikidata,
        name: "OFIP",
        extensions: &["ofip"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
