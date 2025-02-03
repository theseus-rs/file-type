use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_573500267: FileFormat = FileFormat {
    id: 573_500_267,
    source_type: SourceType::Iana,
    name: "parityfec",
    extensions: &[],
    media_types: &["audio/parityfec"],
    internal_signatures: &[],
    related_formats: &[],
};
