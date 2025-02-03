use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2726800377: FileFormat = FileFormat {
    id: 2_726_800_377,
    source_type: SourceType::Iana,
    name: "gff3",
    extensions: &[],
    media_types: &["text/gff3"],
    internal_signatures: &[],
    related_formats: &[],
};
