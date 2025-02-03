use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_814842370: FileFormat = FileFormat {
    id: 814_842_370,
    source_type: SourceType::Httpd,
    name: "yamaha smaf phrase",
    extensions: &["spf"],
    media_types: &["application/vnd.yamaha.smaf-phrase"],
    internal_signatures: &[],
    related_formats: &[],
};
