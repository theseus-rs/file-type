use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859599: FileFormat = FileFormat {
    id: 105_859_599,
    puid: "wikidata/105859599",
    name: "PLC Data",
    extensions: &["vd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4C, 0x43, 0x20, 0x44, 0x61, 0x74, 0x61, 0x20, 0x46, 0x69, 0x6C, 0x65,
                    0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
