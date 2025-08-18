use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_135641981: FileType = FileType {
    file_format: &FileFormat {
        id: 135_641_981,
        source_type: SourceType::Wikidata,
        name: "Ikarus format",
        extensions: &["ik"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
