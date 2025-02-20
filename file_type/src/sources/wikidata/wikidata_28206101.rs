use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206101: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_101,
        source_type: SourceType::Wikidata,
        name: "FaceSaver",
        extensions: &["fac", "face"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
