use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_303387740: FileFormat = FileFormat {
    id: 303_387_740,
    source_type: SourceType::Iana,
    name: "multipart-core",
    extensions: &[],
    media_types: &["application/multipart-core"],
    signatures: &[],
    related_formats: &[],
};
