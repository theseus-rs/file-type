use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114889812: FileType = FileType {
    file_format: &FileFormat {
        id: 114_889_812,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Caledar file",
        extensions: &["scl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
