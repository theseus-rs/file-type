use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136418380: FileType = FileType {
    file_format: &FileFormat {
        id: 136_418_380,
        source_type: SourceType::Wikidata,
        name: "PDF/UA Portable Document Format",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[],
        related_formats: &[],
    },
};
