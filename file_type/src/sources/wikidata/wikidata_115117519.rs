use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_115117519: FileType = FileType {
    file_format: &FileFormat {
        id: 115_117_519,
        source_type: SourceType::Wikidata,
        name: "Help Librarian File",
        extensions: &["dat", "dta", "hlp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
