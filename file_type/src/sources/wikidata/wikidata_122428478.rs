use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122428478: FileType = FileType {
    file_format: &FileFormat {
        id: 122_428_478,
        source_type: SourceType::Wikidata,
        name: "Wild Photo Effects file",
        extensions: &["moo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
