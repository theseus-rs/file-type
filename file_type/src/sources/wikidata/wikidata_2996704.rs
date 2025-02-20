use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_2996704: FileType = FileType {
    file_format: &FileFormat {
        id: 2_996_704,
        source_type: SourceType::Wikidata,
        name: ".htpasswd",
        extensions: &["htpasswd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
