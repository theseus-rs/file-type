use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_399630772: FileFormat = FileFormat {
    id: 399_630_772,
    source_type: SourceType::Iana,
    name: "vnd.ezpix-package",
    extensions: &[],
    media_types: &["application/vnd.ezpix-package"],
    signatures: &[],
    related_formats: &[],
};
