use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123595865: FileType = FileType {
    file_format: &FileFormat {
        id: 123_595_865,
        source_type: SourceType::Wikidata,
        name: "Portable Document Format/Archive, version 4f",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[],
        related_formats: &[],
    },
};
