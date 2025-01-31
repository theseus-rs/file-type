use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866066: FileFormat = FileFormat {
    id: 105_866_066,
    puid: "wikidata/105866066",
    name: "ProShape drawing",
    extensions: &["psp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x72, 0x6F, 0x53, 0x68, 0x61, 0x70, 0x65, 0x20, 0x44, 0x72, 0x61, 0x77,
                    0x69, 0x6E, 0x67, 0x0D, 0x0A, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
