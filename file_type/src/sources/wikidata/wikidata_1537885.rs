use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1537885: FileType = FileType {
    file_format: &FileFormat {
        id: 1_537_885,
        source_type: SourceType::Wikidata,
        name: "Precision Graphics Markup Language",
        extensions: &["pgml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
