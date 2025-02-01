use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857566: FileFormat = FileFormat {
    id: 105_857_566,
    puid: "wikidata/105857566",
    name: "Alpha Four Index Definition",
    extensions: &["idn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x34, 0x06, 0x0A, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
