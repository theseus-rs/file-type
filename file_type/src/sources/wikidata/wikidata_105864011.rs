use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864011: FileFormat = FileFormat {
    id: 105_864_011,
    source_type: SourceType::Wikidata,
    name: "CA-Compete! Model (v4.0)",
    extensions: &["mdl"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x13, 0x00, 0x64, 0x00, 0xCD, 0xAB, 0x43, 0x6F, 0x6D, 0x70, 0x65, 0x74, 0x65,
                    0x21, 0x20, 0x34, 0x2E, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
