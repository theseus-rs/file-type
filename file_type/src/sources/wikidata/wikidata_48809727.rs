use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48809727: FileType = FileType {
    file_format: &FileFormat {
        id: 48_809_727,
        source_type: SourceType::Wikidata,
        name: "dBASE Text Memo",
        extensions: &["dbt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
