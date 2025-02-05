use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865450: FileFormat = FileFormat {
    id: 105_865_450,
    source_type: SourceType::Wikidata,
    name: "Protocol Data Unit message data",
    extensions: &["pdu"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
