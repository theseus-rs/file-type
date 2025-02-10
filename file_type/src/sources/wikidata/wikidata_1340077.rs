use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1340077: FileType = FileType {
    file_format: &FileFormat {
        id: 1_340_077,
        source_type: SourceType::Wikidata,
        name: "Encoded Archival Description",
        extensions: &["sgm", "sgml"],
        media_types: &["text/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
