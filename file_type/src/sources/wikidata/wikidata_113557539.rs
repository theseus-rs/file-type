use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113557539: FileFormat = FileFormat {
    id: 113_557_539,
    source_type: SourceType::Wikidata,
    name: "Prassi CD Right Plus Image",
    extensions: &["gcd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
