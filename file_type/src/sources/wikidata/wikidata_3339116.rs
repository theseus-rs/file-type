use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3339116: FileType = FileType {
    file_format: &FileFormat {
        id: 3_339_116,
        source_type: SourceType::Wikidata,
        name: "Newick tree format",
        extensions: &["newick"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
