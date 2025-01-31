use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862811: FileFormat = FileFormat {
    id: 105_862_811,
    puid: "wikidata/105862811",
    name: "MagicDraw UML project",
    extensions: &["mdr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x61, 0x67, 0x69, 0x63, 0x44, 0x72, 0x61, 0x77, 0x55, 0x4D, 0x4C, 0x20,
                    0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
