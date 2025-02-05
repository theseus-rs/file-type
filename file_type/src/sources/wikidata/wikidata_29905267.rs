use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29905267: FileFormat = FileFormat {
    id: 29_905_267,
    source_type: SourceType::Wikidata,
    name: "Self-Extracting Archive",
    extensions: &["sea"],
    media_types: &["application/x-sea"],
    signatures: &[],
    related_formats: &[],
};
