use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122974666: FileType = FileType {
    file_format: &FileFormat {
        id: 122_974_666,
        source_type: SourceType::Wikidata,
        name: "CAMP",
        extensions: &["camp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
