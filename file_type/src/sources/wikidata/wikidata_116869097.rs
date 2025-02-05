use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116869097: FileFormat = FileFormat {
    id: 116_869_097,
    source_type: SourceType::Wikidata,
    name: "Summitsoft Envelope",
    extensions: &["env"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
