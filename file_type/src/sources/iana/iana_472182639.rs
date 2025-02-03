use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_472182639: FileFormat = FileFormat {
    id: 472_182_639,
    source_type: SourceType::Iana,
    name: "1d-interleaved-parityfec",
    extensions: &[],
    media_types: &["application/1d-interleaved-parityfec"],
    internal_signatures: &[],
    related_formats: &[],
};
