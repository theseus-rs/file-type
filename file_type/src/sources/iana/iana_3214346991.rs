use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3214346991: FileType = FileType {
    file_format: &FileFormat {
        id: 3_214_346_991,
        source_type: SourceType::Iana,
        name: "vnd.fujifilm.fb.docuworks.container",
        extensions: &[],
        media_types: &["application/vnd.fujifilm.fb.docuworks.container"],
        signatures: &[],
        related_formats: &[],
    },
};
