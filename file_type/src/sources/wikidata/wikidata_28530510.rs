use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28530510: FileType = FileType {
    file_format: &FileFormat {
        id: 28_530_510,
        source_type: SourceType::Wikidata,
        name: "Structure-data file",
        extensions: &["sdf"],
        media_types: &["chemical/x-mdl-sdfile"],
        signatures: &[],
        related_formats: &[],
    },
};
