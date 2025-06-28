use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134881767: FileType = FileType {
    file_format: &FileFormat {
        id: 134_881_767,
        source_type: SourceType::Wikidata,
        name: "Soild Edge Sheet",
        extensions: &["psm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
