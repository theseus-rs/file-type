use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1223779917: FileFormat = FileFormat {
    id: 1_223_779_917,
    source_type: SourceType::Httpd,
    name: "smil xml",
    extensions: &["smi", "smil"],
    media_types: &["application/smil+xml"],
    signatures: &[],
    related_formats: &[],
};
