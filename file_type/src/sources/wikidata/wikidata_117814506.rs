use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117814506: FileType = FileType {
    file_format: &FileFormat {
        id: 117_814_506,
        source_type: SourceType::Wikidata,
        name: "Adaptive Information Systems",
        extensions: &["ais"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
