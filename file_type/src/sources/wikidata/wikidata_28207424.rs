use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207424: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_424,
        source_type: SourceType::Wikidata,
        name: "VEGX",
        extensions: &["egx", "vgx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
