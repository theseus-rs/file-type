use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2764815568: FileFormat = FileFormat {
    id: 2_764_815_568,
    source_type: SourceType::Iana,
    name: "report",
    extensions: &[],
    media_types: &["multipart/report"],
    signatures: &[],
    related_formats: &[],
};
