use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100707279: FileType = FileType {
    file_format: &FileFormat {
        id: 100_707_279,
        source_type: SourceType::Wikidata,
        name: "QuarkXPress Project 10.1",
        extensions: &[],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[],
        related_formats: &[],
    },
};
