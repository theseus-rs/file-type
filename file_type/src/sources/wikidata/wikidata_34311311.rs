use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34311311: FileType = FileType {
    file_format: &FileFormat {
        id: 34_311_311,
        source_type: SourceType::Wikidata,
        name: "Sense8 Neutral File Format, plain text variant",
        extensions: &["nff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
