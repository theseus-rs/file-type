use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100323933: FileType = FileType {
    file_format: &FileFormat {
        id: 100_323_933,
        source_type: SourceType::Wikidata,
        name: "GST Publisher File 2",
        extensions: &["dtp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
