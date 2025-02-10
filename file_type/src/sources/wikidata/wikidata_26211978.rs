use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_26211978: FileType = FileType {
    file_format: &FileFormat {
        id: 26_211_978,
        source_type: SourceType::Wikidata,
        name: "ZIP archive file format, version 1.1",
        extensions: &["zip"],
        media_types: &["application/zip"],
        signatures: &[],
        related_formats: &[],
    },
};
