use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_26211840: FileType = FileType {
    file_format: &FileFormat {
        id: 26_211_840,
        source_type: SourceType::Wikidata,
        name: "ZIP archive file format, ISO/IEC 21320–1:2015",
        extensions: &["zip"],
        media_types: &["application/zip"],
        signatures: &[],
        related_formats: &[],
    },
};
