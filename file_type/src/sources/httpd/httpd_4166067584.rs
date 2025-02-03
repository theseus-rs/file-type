use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4166067584: FileFormat = FileFormat {
    id: 4_166_067_584,
    source_type: SourceType::Httpd,
    name: "mediaservercontrol xml",
    extensions: &["mscml"],
    media_types: &["application/mediaservercontrol+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
