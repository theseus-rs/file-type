use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_34747392: FileType = FileType {
    file_format: &FileFormat {
        id: 34_747_392,
        source_type: SourceType::Wikidata,
        name: "STATISTICA SNN",
        extensions: &["snn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
