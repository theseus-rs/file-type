use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2662883759: FileFormat = FileFormat {
    id: 2_662_883_759,
    source_type: SourceType::Httpd,
    name: "adobe photoshop",
    extensions: &["psd"],
    media_types: &["image/vnd.adobe.photoshop"],
    internal_signatures: &[],
    related_formats: &[],
};
