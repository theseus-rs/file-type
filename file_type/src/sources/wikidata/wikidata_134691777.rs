use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134691777: FileType = FileType {
    file_format: &FileFormat {
        id: 134_691_777,
        source_type: SourceType::Wikidata,
        name: "NooJ dictionary file",
        extensions: &["dic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
