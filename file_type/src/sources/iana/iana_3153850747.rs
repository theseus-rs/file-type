use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3153850747: FileFormat = FileFormat {
    id: 3_153_850_747,
    source_type: SourceType::Iana,
    name: "vnd.exstream-empower+zip",
    extensions: &[],
    media_types: &["application/vnd.exstream-empower+zip"],
    internal_signatures: &[],
    related_formats: &[],
};
