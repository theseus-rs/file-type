use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_26211940: FileType = FileType {
    file_format: &FileFormat {
        id: 26_211_940,
        source_type: SourceType::Wikidata,
        name: "ZIP archive file format, version 5.1",
        extensions: &["zip"],
        media_types: &["application/zip"],
        signatures: &[],
        related_formats: &[],
    },
};
