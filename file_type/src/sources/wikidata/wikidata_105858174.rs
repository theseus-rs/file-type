use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858174: FileFormat = FileFormat {
    id: 105_858_174,
    puid: "wikidata/105858174",
    name: "Electronic Music System v6 module",
    extensions: &["ems"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x2E, 0x4D, 0x2E, 0x53, 0x2E, 0x20, 0x56, 0x36, 0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
