use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28205503: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_503,
        source_type: SourceType::Wikidata,
        name: "GEM resource file",
        extensions: &["rsc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
