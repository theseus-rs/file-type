use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1272260793: FileFormat = FileFormat {
    id: 1_272_260_793,
    source_type: SourceType::Iana,
    name: "cql-identifier",
    extensions: &[],
    media_types: &["text/cql-identifier"],
    signatures: &[],
    related_formats: &[],
};
