use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862244: FileFormat = FileFormat {
    id: 105_862_244,
    puid: "wikidata/105862244",
    name: "Blizzard 3D Model (binary)",
    extensions: &["mdx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x44, 0x4C, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
