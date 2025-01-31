use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858743: FileFormat = FileFormat {
    id: 105_858_743,
    puid: "wikidata/105858743",
    name: "Paintpro bitmap (generic)",
    extensions: &["ppp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x41, 0x49, 0x4E, 0x54, 0x50, 0x52, 0x4F, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
