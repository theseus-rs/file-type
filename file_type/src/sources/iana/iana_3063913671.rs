use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3063913671: FileFormat = FileFormat {
    id: 3_063_913_671,
    source_type: SourceType::Iana,
    name: "vnd.imagemeter.image+zip",
    extensions: &[],
    media_types: &["application/vnd.imagemeter.image+zip"],
    signatures: &[],
    related_formats: &[],
};
