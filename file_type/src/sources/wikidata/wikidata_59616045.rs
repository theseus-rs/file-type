use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59616045: FileType = FileType {
    file_format: &FileFormat {
        id: 59_616_045,
        source_type: SourceType::Wikidata,
        name: "Zope export file",
        extensions: &["zexp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
