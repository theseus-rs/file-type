use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47490002: FileType = FileType {
    file_format: &FileFormat {
        id: 47_490_002,
        source_type: SourceType::Wikidata,
        name: "Adobe FrameMaker Document, version 7",
        extensions: &["fm"],
        media_types: &["application/vnd.framemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
