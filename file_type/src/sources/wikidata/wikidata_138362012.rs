use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138362012: FileType = FileType {
    file_format: &FileFormat {
        id: 138_362_012,
        source_type: SourceType::Wikidata,
        name: "ACE 2",
        extensions: &["ace"],
        media_types: &["application/x-ace-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
