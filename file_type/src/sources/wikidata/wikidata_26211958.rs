use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26211958: FileType = FileType {
    file_format: &FileFormat {
        id: 26_211_958,
        source_type: SourceType::Wikidata,
        name: "ZIP archive file format, version 2.7",
        extensions: &["zip"],
        media_types: &["application/zip"],
        signatures: &[],
        related_formats: &[],
    },
};
