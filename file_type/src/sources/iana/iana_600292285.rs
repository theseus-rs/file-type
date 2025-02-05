use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_600292285: FileFormat = FileFormat {
    id: 600_292_285,
    source_type: SourceType::Iana,
    name: "prs.plucker",
    extensions: &[],
    media_types: &["application/prs.plucker"],
    signatures: &[],
    related_formats: &[],
};
