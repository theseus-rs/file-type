use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34311506: FileType = FileType {
    file_format: &FileFormat {
        id: 34_311_506,
        source_type: SourceType::Wikidata,
        name: "Sense8 Neutral File Format, binary variant",
        extensions: &["bff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
