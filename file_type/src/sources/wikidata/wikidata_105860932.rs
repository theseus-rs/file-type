use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860932: FileFormat = FileFormat {
    id: 105_860_932,
    puid: "wikidata/105860932",
    name: "Alpha Four field rules",
    extensions: &["rln"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x34, 0x08, 0x0A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
