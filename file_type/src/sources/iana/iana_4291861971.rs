use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4291861971: FileFormat = FileFormat {
    id: 4_291_861_971,
    source_type: SourceType::Iana,
    name: "vnd.cups-ppd",
    extensions: &[],
    media_types: &["application/vnd.cups-ppd"],
    internal_signatures: &[],
    related_formats: &[],
};
