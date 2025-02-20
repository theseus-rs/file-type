use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122018104: FileType = FileType {
    file_format: &FileFormat {
        id: 122_018_104,
        source_type: SourceType::Wikidata,
        name: "Stationery Brochures and More Document",
        extensions: &["dtp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
