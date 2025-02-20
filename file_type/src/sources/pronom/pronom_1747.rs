use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1747: FileType = FileType {
    file_format: &FileFormat {
        id: 1_747,
        source_type: SourceType::Pronom,
        name: "Adobe Air",
        extensions: &["air"],
        media_types: &["application/vnd.adobe.air-application-installer-package+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
