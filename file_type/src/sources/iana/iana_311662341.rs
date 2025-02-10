use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_311662341: FileType = FileType {
    file_format: &FileFormat {
        id: 311_662_341,
        source_type: SourceType::Iana,
        name: "vnd.exstream-package",
        extensions: &[],
        media_types: &["application/vnd.exstream-package"],
        signatures: &[],
        related_formats: &[],
    },
};
