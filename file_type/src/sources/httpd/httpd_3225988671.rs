use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3225988671: FileFormat = FileFormat {
    id: 3_225_988_671,
    source_type: SourceType::Httpd,
    name: "3gpp2",
    extensions: &["3g2"],
    media_types: &["video/3gpp2"],
    internal_signatures: &[],
    related_formats: &[],
};
