use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862467: FileFormat = FileFormat {
    id: 105_862_467,
    puid: "wikidata/105862467",
    name: "StarTrekker 4-channel module",
    extensions: &["mod"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x4C, 0x54, 0x34])],
            },
        }],
    }],
    related_formats: &[],
};
