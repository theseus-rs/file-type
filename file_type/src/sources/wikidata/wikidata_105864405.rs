use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864405: FileFormat = FileFormat {
    id: 105_864_405,
    puid: "wikidata/105864405",
    name: "PHREEQC data",
    extensions: &["pqo"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x20, 0x20, 0x20, 0x49, 0x6E, 0x70, 0x75, 0x74, 0x20, 0x66, 0x69, 0x6C, 0x65,
                    0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
