use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854994: FileFormat = FileFormat {
    id: 105_854_994,
    puid: "wikidata/105854994",
    name: "AMOS source (v1.00)",
    extensions: &["amos"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x4F, 0x53, 0x20, 0x42, 0x61, 0x73, 0x69, 0x63, 0x20, 0x56, 0x31,
                    0x2E, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
