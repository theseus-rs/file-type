use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866360: FileFormat = FileFormat {
    id: 105_866_360,
    puid: "wikidata/105866360",
    name: "Boeing Calc WorkPad (v3.x)",
    extensions: &["pad"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x41, 0x4C, 0x43, 0x33, 0x2E])],
            },
        }],
    }],
    related_formats: &[],
};
