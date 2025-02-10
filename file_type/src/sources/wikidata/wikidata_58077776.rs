use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58077776: FileType = FileType {
    file_format: &FileFormat {
        id: 58_077_776,
        source_type: SourceType::Wikidata,
        name: "AbiWord Document Template",
        extensions: &["awt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
