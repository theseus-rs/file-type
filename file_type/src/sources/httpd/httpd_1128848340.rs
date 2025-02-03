use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1128848340: FileFormat = FileFormat {
    id: 1_128_848_340,
    source_type: SourceType::Httpd,
    name: "adpcm",
    extensions: &["adp"],
    media_types: &["audio/adpcm"],
    internal_signatures: &[],
    related_formats: &[],
};
