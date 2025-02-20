use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_65533440: FileType = FileType {
    file_format: &FileFormat {
        id: 65_533_440,
        source_type: SourceType::Wikidata,
        name: "BigOven Recipe Box File",
        extensions: &["crb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
