use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131153470: FileType = FileType {
    file_format: &FileFormat {
        id: 131_153_470,
        source_type: SourceType::Wikidata,
        name: "sqlite3con file format",
        extensions: &["sqlite3-console"],
        media_types: &["text/x-sqlite3-console"],
        signatures: &[],
        related_formats: &[],
    },
};
