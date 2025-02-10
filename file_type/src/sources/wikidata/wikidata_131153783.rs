use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131153783: FileType = FileType {
    file_format: &FileFormat {
        id: 131_153_783,
        source_type: SourceType::Wikidata,
        name: "squid configuration file format",
        extensions: &["squid.conf"],
        media_types: &["text/x-squidconf"],
        signatures: &[],
        related_formats: &[],
    },
};
