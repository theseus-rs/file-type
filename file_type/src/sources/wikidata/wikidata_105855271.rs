use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855271: FileFormat = FileFormat {
    id: 105_855_271,
    puid: "wikidata/105855271",
    name: "FireFly menu definition",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x46, 0x49, 0x52, 0x45, 0x46, 0x4C, 0x59, 0x5F, 0x54, 0x4F, 0x50, 0x4D,
                    0x45, 0x4E, 0x55, 0x23, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
