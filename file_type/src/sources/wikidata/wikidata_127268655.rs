use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127268655: FileType = FileType {
    file_format: &FileFormat {
        id: 127_268_655,
        source_type: SourceType::Wikidata,
        name: "CATIA V5 Elysium Neutral File",
        extensions: &["enf_abq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
