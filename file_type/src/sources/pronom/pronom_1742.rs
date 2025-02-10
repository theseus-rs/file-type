use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1742: FileType = FileType {
    file_format: &FileFormat {
        id: 1_742,
        source_type: SourceType::Pronom,
        name: "Adobe Air",
        extensions: &["air"],
        media_types: &["application/vnd.adobe.air-application-installer-package+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
