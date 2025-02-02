use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10233703363409414994: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "adobe photoshop",
    extensions: &["psd"],
    media_types: &["image/vnd.adobe.photoshop"],
    internal_signatures: &[],
    related_formats: &[],
};
