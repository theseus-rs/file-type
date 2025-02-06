use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_115736167: FileFormat = FileFormat {
    id: 115_736_167,
    source_type: SourceType::Iana,
    name: "vnd.wolfram.mathematica.package",
    extensions: &[],
    media_types: &["application/vnd.wolfram.mathematica.package"],
    signatures: &[],
    related_formats: &[],
};
