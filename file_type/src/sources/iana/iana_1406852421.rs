use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1406852421: FileType = FileType {
    file_format: &FileFormat {
        id: 1_406_852_421,
        source_type: SourceType::Iana,
        name: "vnd.opengex",
        extensions: &[],
        media_types: &["model/vnd.opengex"],
        signatures: &[],
        related_formats: &[],
    },
};
