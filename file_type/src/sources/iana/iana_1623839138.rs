use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1623839138: FileFormat = FileFormat {
    id: 1_623_839_138,
    source_type: SourceType::Iana,
    name: "1d-interleaved-parityfec",
    extensions: &[],
    media_types: &["text/1d-interleaved-parityfec"],
    internal_signatures: &[],
    related_formats: &[],
};
