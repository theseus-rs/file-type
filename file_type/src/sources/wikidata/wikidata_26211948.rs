use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26211948: FileType = FileType {
    file_format: &FileFormat {
        id: 26_211_948,
        source_type: SourceType::Wikidata,
        name: "ZIP archive file format, version 5.0",
        extensions: &["zip"],
        media_types: &["application/zip"],
        signatures: &[],
        related_formats: &[],
    },
};
