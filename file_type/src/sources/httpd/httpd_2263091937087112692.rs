use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2263091937087112692: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "adpcm",
    extensions: &["adp"],
    media_types: &["audio/adpcm"],
    internal_signatures: &[],
    related_formats: &[],
};
