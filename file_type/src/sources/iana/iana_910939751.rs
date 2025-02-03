use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_910939751: FileFormat = FileFormat {
    id: 910_939_751,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.mc-signalling-ear",
    extensions: &[],
    media_types: &["application/vnd.3gpp.mc-signalling-ear"],
    internal_signatures: &[],
    related_formats: &[],
};
