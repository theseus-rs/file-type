use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858835: FileFormat = FileFormat {
    id: 105_858_835,
    puid: "wikidata/105858835",
    name: "Blowfish Advanced CS encrypted",
    extensions: &["bfa"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x24, 0x08, 0x19, 0x92, 0x23, 0x00, 0x15, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
