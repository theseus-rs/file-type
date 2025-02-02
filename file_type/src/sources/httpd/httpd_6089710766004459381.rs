use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6089710766004459381: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fastbidsheet",
    extensions: &["fbs"],
    media_types: &["image/vnd.fastbidsheet"],
    internal_signatures: &[],
    related_formats: &[],
};
