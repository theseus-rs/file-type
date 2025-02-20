use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34303668: FileType = FileType {
    file_format: &FileFormat {
        id: 34_303_668,
        source_type: SourceType::Wikidata,
        name: "Syntactically Awesome StyleSheet",
        extensions: &["sass"],
        media_types: &["text/x-sass"],
        signatures: &[],
        related_formats: &[],
    },
};
