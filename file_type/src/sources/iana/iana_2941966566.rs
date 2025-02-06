use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2941966566: FileFormat = FileFormat {
    id: 2_941_966_566,
    source_type: SourceType::Iana,
    name: "prs.texi",
    extensions: &[],
    media_types: &["text/prs.texi"],
    signatures: &[],
    related_formats: &[],
};
