use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3748682725: FileType = FileType {
    file_format: &FileFormat {
        id: 3_748_682_725,
        source_type: SourceType::Iana,
        name: "vnd.collabio.xodocuments.presentation-template",
        extensions: &[],
        media_types: &["application/vnd.collabio.xodocuments.presentation-template"],
        signatures: &[],
        related_formats: &[],
    },
};
