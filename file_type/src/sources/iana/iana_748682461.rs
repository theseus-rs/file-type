use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_748682461: FileType = FileType {
    file_format: &FileFormat {
        id: 748_682_461,
        source_type: SourceType::Iana,
        name: "vnd.doremir.scorecloud-binary-document",
        extensions: &[],
        media_types: &["application/vnd.doremir.scorecloud-binary-document"],
        signatures: &[],
        related_formats: &[],
    },
};
