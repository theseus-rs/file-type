use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_17144293: FileType = FileType {
    file_format: &FileFormat {
        id: 17_144_293,
        source_type: SourceType::Wikidata,
        name: "UBJSON",
        extensions: &["ubj"],
        media_types: &["application/ubjson"],
        signatures: &[],
        related_formats: &[],
    },
};
