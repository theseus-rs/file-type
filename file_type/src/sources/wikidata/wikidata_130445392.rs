use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130445392: FileType = FileType {
    file_format: &FileFormat {
        id: 130_445_392,
        source_type: SourceType::Wikidata,
        name: "GnuCash XML Format",
        extensions: &["gnucash"],
        media_types: &["application/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
