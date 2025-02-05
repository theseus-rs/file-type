use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856056: FileFormat = FileFormat {
    id: 105_856_056,
    source_type: SourceType::Wikidata,
    name: "Dynamix game data archive",
    extensions: &["dyn"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x79, 0x6E, 0x61, 0x6D, 0x69, 0x78, 0x20, 0x56, 0x6F, 0x6C, 0x75, 0x6D,
                    0x65, 0x20, 0x46, 0x69, 0x6C, 0x65, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
