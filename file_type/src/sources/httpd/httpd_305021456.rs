use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_305021456: FileFormat = FileFormat {
    id: 305_021_456,
    source_type: SourceType::Httpd,
    name: "novadigm edx",
    extensions: &["edx"],
    media_types: &["application/vnd.novadigm.edx"],
    internal_signatures: &[],
    related_formats: &[],
};
