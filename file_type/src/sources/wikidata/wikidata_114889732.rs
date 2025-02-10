use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114889732: FileType = FileType {
    file_format: &FileFormat {
        id: 114_889_732,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Cover file",
        extensions: &["ssc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
