use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3449934334: FileFormat = FileFormat {
    id: 3_449_934_334,
    source_type: SourceType::Iana,
    name: "vnd.shana.informed.formtemplate",
    extensions: &[],
    media_types: &["application/vnd.shana.informed.formtemplate"],
    signatures: &[],
    related_formats: &[],
};
