use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858755: FileFormat = FileFormat {
    id: 105_858_755,
    puid: "wikidata/105858755",
    name: "Paintpro bitmap (v6.0)",
    extensions: &["ppp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x41, 0x49, 0x4E, 0x54, 0x50, 0x52, 0x4F, 0x56, 0x36, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
