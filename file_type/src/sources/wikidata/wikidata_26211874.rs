use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26211874: FileType = FileType {
    file_format: &FileFormat {
        id: 26_211_874,
        source_type: SourceType::Wikidata,
        name: "ZIP archive file format, version 6.3.1",
        extensions: &["zip"],
        media_types: &["application/zip"],
        signatures: &[],
        related_formats: &[],
    },
};
