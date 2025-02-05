use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_522531130: FileFormat = FileFormat {
    id: 522_531_130,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-package.relationships+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-package.relationships+xml"],
    signatures: &[],
    related_formats: &[],
};
