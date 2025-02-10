use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_2713137: FileType = FileType {
    file_format: &FileFormat {
        id: 2_713_137,
        source_type: SourceType::Wikidata,
        name: "Crystallographic Information File",
        extensions: &["cif"],
        media_types: &["chemical/x-cif"],
        signatures: &[],
        related_formats: &[],
    },
};
