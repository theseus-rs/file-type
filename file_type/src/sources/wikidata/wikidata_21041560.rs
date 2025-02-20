use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_21041560: FileType = FileType {
    file_format: &FileFormat {
        id: 21_041_560,
        source_type: SourceType::Wikidata,
        name: "Oktalyzer format",
        extensions: &["okt", "okta"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
