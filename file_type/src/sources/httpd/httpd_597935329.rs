use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_597935329: FileFormat = FileFormat {
    id: 597_935_329,
    source_type: SourceType::Httpd,
    name: "ds keypoint",
    extensions: &["kpxx"],
    media_types: &["application/vnd.ds-keypoint"],
    signatures: &[],
    related_formats: &[],
};
