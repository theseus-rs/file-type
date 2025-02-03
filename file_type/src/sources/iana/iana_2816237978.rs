use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2816237978: FileFormat = FileFormat {
    id: 2_816_237_978,
    source_type: SourceType::Iana,
    name: "prc",
    extensions: &[],
    media_types: &["model/prc"],
    internal_signatures: &[],
    related_formats: &[],
};
