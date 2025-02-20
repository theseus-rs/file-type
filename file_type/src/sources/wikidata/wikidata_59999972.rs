use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59999972: FileType = FileType {
    file_format: &FileFormat {
        id: 59_999_972,
        source_type: SourceType::Wikidata,
        name: "Borland Reflex flat datafile",
        extensions: &["rxd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
