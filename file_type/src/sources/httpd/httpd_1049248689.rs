use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1049248689: FileFormat = FileFormat {
    id: 1_049_248_689,
    source_type: SourceType::Httpd,
    name: "dece hd",
    extensions: &["uvh", "uvvh"],
    media_types: &["video/vnd.dece.hd"],
    internal_signatures: &[],
    related_formats: &[],
};
