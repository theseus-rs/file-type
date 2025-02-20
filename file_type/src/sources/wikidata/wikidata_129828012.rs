use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_129828012: FileType = FileType {
    file_format: &FileFormat {
        id: 129_828_012,
        source_type: SourceType::Wikidata,
        name: "Ioke source code file",
        extensions: &["ik"],
        media_types: &["text/x-iokesrc"],
        signatures: &[],
        related_formats: &[],
    },
};
