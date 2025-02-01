use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860831: FileFormat = FileFormat {
    id: 105_860_831,
    puid: "wikidata/105860831",
    name: "Rob Northen Compression (type 2)",
    extensions: &["rnc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x4E, 0x43, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
