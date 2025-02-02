use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7052973897628710533: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ds keypoint",
    extensions: &["kpxx"],
    media_types: &["application/vnd.ds-keypoint"],
    internal_signatures: &[],
    related_formats: &[],
};
