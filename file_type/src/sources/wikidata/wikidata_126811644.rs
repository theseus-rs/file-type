use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126811644: FileType = FileType {
    file_format: &FileFormat {
        id: 126_811_644,
        source_type: SourceType::Wikidata,
        name: "Fenix Graphics Collection File",
        extensions: &["fpg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
