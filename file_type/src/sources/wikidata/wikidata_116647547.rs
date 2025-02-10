use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116647547: FileType = FileType {
    file_format: &FileFormat {
        id: 116_647_547,
        source_type: SourceType::Wikidata,
        name: "Form file",
        extensions: &["ofm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
