use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131453612: FileType = FileType {
    file_format: &FileFormat {
        id: 131_453_612,
        source_type: SourceType::Wikidata,
        name: "Zeek file format",
        extensions: &["bro", "zeek"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
