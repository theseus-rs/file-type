use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853700: FileFormat = FileFormat {
    id: 105_853_700,
    puid: "wikidata/105853700",
    name: "EditPlus Auto Completion",
    extensions: &["acp"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x54, 0x49, 0x54, 0x4C, 0x45, 0x3D])],
            },
        }],
    }],
    related_formats: &[],
};
