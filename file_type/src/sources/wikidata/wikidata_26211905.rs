use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_26211905: FileType = FileType {
    file_format: &FileFormat {
        id: 26_211_905,
        source_type: SourceType::Wikidata,
        name: "ZIP archive file format, version 6.2.2",
        extensions: &["zip"],
        media_types: &["application/zip"],
        signatures: &[],
        related_formats: &[],
    },
};
