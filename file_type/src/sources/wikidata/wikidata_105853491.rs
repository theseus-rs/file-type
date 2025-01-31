use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853491: FileFormat = FileFormat {
    id: 105_853_491,
    puid: "wikidata/105853491",
    name: "HLGuard Z config file",
    extensions: &["zcfg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x55, 0x41, 0x2D, 0x63, 0x66, 0x67])],
            },
        }],
    }],
    related_formats: &[],
};
