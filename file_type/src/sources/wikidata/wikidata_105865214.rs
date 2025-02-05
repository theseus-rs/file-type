use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865214: FileFormat = FileFormat {
    id: 105_865_214,
    source_type: SourceType::Wikidata,
    name: "Password Depot 6 data",
    extensions: &["psw6"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0E, 0x50, 0x53, 0x57, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x20, 0x76, 0x2E, 0x36,
                    0x2E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
