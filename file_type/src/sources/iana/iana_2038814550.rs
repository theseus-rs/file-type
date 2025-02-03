use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2038814550: FileFormat = FileFormat {
    id: 2_038_814_550,
    source_type: SourceType::Iana,
    name: "1d-interleaved-parityfec",
    extensions: &[],
    media_types: &["video/1d-interleaved-parityfec"],
    internal_signatures: &[],
    related_formats: &[],
};
