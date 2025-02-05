use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_511568138: FileFormat = FileFormat {
    id: 511_568_138,
    source_type: SourceType::Iana,
    name: "vnd.geogebra.file",
    extensions: &[],
    media_types: &["application/vnd.geogebra.file"],
    signatures: &[],
    related_formats: &[],
};
