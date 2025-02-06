use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_935809: FileFormat = FileFormat {
    id: 935_809,
    source_type: SourceType::Wikidata,
    name: "comma-separated values",
    extensions: &["csv"],
    media_types: &["text/csv"],
    signatures: &[],
    related_formats: &[],
};
