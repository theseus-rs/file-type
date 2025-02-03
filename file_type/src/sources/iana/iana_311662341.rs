use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_311662341: FileFormat = FileFormat {
    id: 311_662_341,
    source_type: SourceType::Iana,
    name: "vnd.exstream-package",
    extensions: &[],
    media_types: &["application/vnd.exstream-package"],
    internal_signatures: &[],
    related_formats: &[],
};
