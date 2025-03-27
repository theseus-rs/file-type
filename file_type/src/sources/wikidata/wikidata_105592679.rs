use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105592679: FileType = FileType {
    file_format: &FileFormat {
        id: 105_592_679,
        source_type: SourceType::Wikidata,
        name: "Guitar Pro 5 tablature",
        extensions: &["gp5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
