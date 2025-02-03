use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_152912554: FileFormat = FileFormat {
    id: 152_912_554,
    source_type: SourceType::Iana,
    name: "vnd.semf",
    extensions: &[],
    media_types: &["application/vnd.semf"],
    internal_signatures: &[],
    related_formats: &[],
};
