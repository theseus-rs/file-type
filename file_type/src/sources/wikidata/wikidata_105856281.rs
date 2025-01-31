use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856281: FileFormat = FileFormat {
    id: 105_856_281,
    puid: "wikidata/105856281",
    name: "GLBasic 3D data",
    extensions: &["ddd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x17, 0xB3, 0xC6, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
