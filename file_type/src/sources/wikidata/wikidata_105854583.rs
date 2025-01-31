use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854583: FileFormat = FileFormat {
    id: 105_854_583,
    puid: "wikidata/105854583",
    name: "Transform compressed",
    extensions: &["tfm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x72, 0x61, 0x6E, 0x73, 0x66, 0x6F, 0x72, 0x6D, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
