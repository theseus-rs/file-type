use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855638: FileFormat = FileFormat {
    id: 105_855_638,
    puid: "wikidata/105855638",
    name: "Oberon V3 Symbol data",
    extensions: &["sym"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x59, 0x4D, 0x3A, 0x30, 0x30, 0x31, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
