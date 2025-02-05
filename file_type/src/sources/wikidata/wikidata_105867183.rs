use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867183: FileFormat = FileFormat {
    id: 105_867_183,
    source_type: SourceType::Wikidata,
    name: "Nokia phone BackUp",
    extensions: &["nbu"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xCC, 0x52, 0x33, 0xFC, 0xE9, 0x2C, 0x18, 0x48, 0xAF, 0xE3, 0x36, 0x30, 0x1A,
                    0x39, 0x40, 0x06, 0x04, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
