use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34287064: FileType = FileType {
    file_format: &FileFormat {
        id: 34_287_064,
        source_type: SourceType::Wikidata,
        name: "Professor Calhoon quiz file",
        extensions: &["pc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
