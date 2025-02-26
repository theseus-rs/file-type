use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132641461: FileType = FileType {
    file_format: &FileFormat {
        id: 132_641_461,
        source_type: SourceType::Wikidata,
        name: "Package binary file",
        extensions: &["pkb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
