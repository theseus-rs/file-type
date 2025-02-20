use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116957974: FileType = FileType {
    file_format: &FileFormat {
        id: 116_957_974,
        source_type: SourceType::Wikidata,
        name: "ICN AT&T/Multigen",
        extensions: &["icn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
