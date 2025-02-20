use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29650512: FileType = FileType {
    file_format: &FileFormat {
        id: 29_650_512,
        source_type: SourceType::Wikidata,
        name: "packJPG",
        extensions: &["pjg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
