use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47512572: FileType = FileType {
    file_format: &FileFormat {
        id: 47_512_572,
        source_type: SourceType::Wikidata,
        name: "man page",
        extensions: &["1", "2", "3", "4", "5", "6", "7", "8", "man"],
        media_types: &["application/x-troff-man", "text/troff"],
        signatures: &[],
        related_formats: &[],
    },
};
