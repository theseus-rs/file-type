use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27959896: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_896,
        source_type: SourceType::Wikidata,
        name: "Nuendo arrangement",
        extensions: &["npr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
