use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111317150: FileType = FileType {
    file_format: &FileFormat {
        id: 111_317_150,
        source_type: SourceType::Wikidata,
        name: "Korg T-series wave",
        extensions: &["kft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
