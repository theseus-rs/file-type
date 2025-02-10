use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29651047: FileType = FileType {
    file_format: &FileFormat {
        id: 29_651_047,
        source_type: SourceType::Wikidata,
        name: "PaperPort",
        extensions: &["max"],
        media_types: &["application/x-paperport"],
        signatures: &[],
        related_formats: &[],
    },
};
