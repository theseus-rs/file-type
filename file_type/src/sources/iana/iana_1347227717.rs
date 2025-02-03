use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1347227717: FileFormat = FileFormat {
    id: 1_347_227_717,
    source_type: SourceType::Iana,
    name: "vnd.oma.bcast.associated-procedure-parameter+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.bcast.associated-procedure-parameter+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
