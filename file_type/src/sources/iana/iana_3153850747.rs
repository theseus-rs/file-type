use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
