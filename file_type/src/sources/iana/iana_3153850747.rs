use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3153850747: FileType = FileType {
    file_format: &FileFormat {
        id: 3_153_850_747,
        source_type: SourceType::Iana,
        name: "vnd.exstream-empower+zip",
        extensions: &[],
        media_types: &["application/vnd.exstream-empower+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
