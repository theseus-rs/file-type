use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66141873: FileType = FileType {
    file_format: &FileFormat {
        id: 66_141_873,
        source_type: SourceType::Wikidata,
        name: "MDE file format",
        extensions: &["mde"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
