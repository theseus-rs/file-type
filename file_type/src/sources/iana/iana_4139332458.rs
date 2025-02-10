use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4139332458: FileType = FileType {
    file_format: &FileFormat {
        id: 4_139_332_458,
        source_type: SourceType::Iana,
        name: "vnd.fujifilm.fb.docuworks.binder",
        extensions: &[],
        media_types: &["application/vnd.fujifilm.fb.docuworks.binder"],
        signatures: &[],
        related_formats: &[],
    },
};
