use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_109944567: FileFormat = FileFormat {
    id: 109_944_567,
    source_type: SourceType::Wikidata,
    name: "Generic CADD file format",
    extensions: &["gcd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
