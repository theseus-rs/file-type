use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47490016: FileType = FileType {
    file_format: &FileFormat {
        id: 47_490_016,
        source_type: SourceType::Wikidata,
        name: "Adobe FrameMaker Document, version 9",
        extensions: &["fm"],
        media_types: &["application/vnd.framemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
