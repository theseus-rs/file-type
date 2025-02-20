use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1335481394: FileType = FileType {
    file_format: &FileFormat {
        id: 1_335_481_394,
        source_type: SourceType::Iana,
        name: "vnd.noblenet-directory",
        extensions: &[],
        media_types: &["application/vnd.noblenet-directory"],
        signatures: &[],
        related_formats: &[],
    },
};
