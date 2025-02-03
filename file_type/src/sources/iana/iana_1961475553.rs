use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1961475553: FileFormat = FileFormat {
    id: 1_961_475_553,
    source_type: SourceType::Iana,
    name: "vnd.Mobius.DIS",
    extensions: &[],
    media_types: &["application/vnd.Mobius.DIS"],
    internal_signatures: &[],
    related_formats: &[],
};
