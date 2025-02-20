use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29000735: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_735,
        source_type: SourceType::Wikidata,
        name: "VOL",
        extensions: &["vol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
