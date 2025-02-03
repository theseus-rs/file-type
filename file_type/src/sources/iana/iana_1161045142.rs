use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1161045142: FileFormat = FileFormat {
    id: 1_161_045_142,
    source_type: SourceType::Iana,
    name: "yang",
    extensions: &[],
    media_types: &["application/yang"],
    internal_signatures: &[],
    related_formats: &[],
};
