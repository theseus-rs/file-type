use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114132929: FileType = FileType {
    file_format: &FileFormat {
        id: 114_132_929,
        source_type: SourceType::Wikidata,
        name: "Constraint File Format",
        extensions: &["con"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
