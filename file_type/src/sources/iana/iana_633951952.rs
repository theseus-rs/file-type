use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_633951952: FileType = FileType {
    file_format: &FileFormat {
        id: 633_951_952,
        source_type: SourceType::Iana,
        name: "vnd.wmc",
        extensions: &[],
        media_types: &["application/vnd.wmc"],
        signatures: &[],
        related_formats: &[],
    },
};
