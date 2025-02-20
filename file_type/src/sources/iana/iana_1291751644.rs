use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1291751644: FileType = FileType {
    file_format: &FileFormat {
        id: 1_291_751_644,
        source_type: SourceType::Iana,
        name: "vnd.fujixerox.docuworks.binder",
        extensions: &[],
        media_types: &["application/vnd.fujixerox.docuworks.binder"],
        signatures: &[],
        related_formats: &[],
    },
};
