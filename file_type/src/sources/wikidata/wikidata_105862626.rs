use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105862626: FileFormat = FileFormat {
    id: 105_862_626,
    puid: "wikidata/105862626",
    name: "MegaCAD Macro",
    extensions: &["mac"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x65, 0x67, 0x61, 0x56, 0x6F, 0x6C])],
            },
        }],
    }],
    related_formats: &[],
};
