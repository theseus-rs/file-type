use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2595581998: FileFormat = FileFormat {
    id: 2_595_581_998,
    source_type: SourceType::Httpd,
    name: "ms wax",
    extensions: &["wax"],
    media_types: &["audio/x-ms-wax"],
    signatures: &[],
    related_formats: &[],
};
