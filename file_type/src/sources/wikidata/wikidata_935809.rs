use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_935809: FileFormat = FileFormat {
    id: 935_809,
    puid: "wikidata/935809",
    name: "comma-separated values",
    extensions: &["csv"],
    media_types: &["text/csv"],
    internal_signatures: &[],
    related_formats: &[],
};
