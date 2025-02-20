use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27960787: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_787,
        source_type: SourceType::Wikidata,
        name: "HS2",
        extensions: &["hs2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
