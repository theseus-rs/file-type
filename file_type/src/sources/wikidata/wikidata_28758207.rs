use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28758207: FileType = FileType {
    file_format: &FileFormat {
        id: 28_758_207,
        source_type: SourceType::Wikidata,
        name: "Adaptive Prediction Trees",
        extensions: &["apt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
