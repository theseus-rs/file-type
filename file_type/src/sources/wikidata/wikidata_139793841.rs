use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_139793841: FileType = FileType {
    file_format: &FileFormat {
        id: 139_793_841,
        source_type: SourceType::Wikidata,
        name: "DirectShow Filter",
        extensions: &["ax"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
