use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116878054: FileType = FileType {
    file_format: &FileFormat {
        id: 116_878_054,
        source_type: SourceType::Wikidata,
        name: "Address Book Text",
        extensions: &["AB5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
