use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967091: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_091,
        source_type: SourceType::Wikidata,
        name: "Funcom ISS",
        extensions: &["iss", "xarc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
