use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116647695: FileType = FileType {
    file_format: &FileFormat {
        id: 116_647_695,
        source_type: SourceType::Wikidata,
        name: "KeyForm 4.0 Document",
        extensions: &["kfm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
