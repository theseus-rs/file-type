use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
