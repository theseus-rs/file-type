use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28205526: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_526,
        source_type: SourceType::Wikidata,
        name: "Icon library",
        extensions: &["icl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
