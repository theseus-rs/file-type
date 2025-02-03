use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2891073491: FileFormat = FileFormat {
    id: 2_891_073_491,
    source_type: SourceType::Iana,
    name: "cellml+xml",
    extensions: &[],
    media_types: &["application/cellml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
