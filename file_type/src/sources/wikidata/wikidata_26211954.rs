use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_26211954: FileType = FileType {
    file_format: &FileFormat {
        id: 26_211_954,
        source_type: SourceType::Wikidata,
        name: "ZIP archive file format, version 4.6",
        extensions: &["zip"],
        media_types: &["application/zip"],
        signatures: &[],
        related_formats: &[],
    },
};
