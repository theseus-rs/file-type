use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1165116: FileType = FileType {
    file_format: &FileFormat {
        id: 1_165_116,
        source_type: SourceType::Wikidata,
        name: "Perl module",
        extensions: &["pm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
