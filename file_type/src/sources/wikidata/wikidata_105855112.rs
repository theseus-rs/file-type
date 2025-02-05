use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855112: FileFormat = FileFormat {
    id: 105_855_112,
    source_type: SourceType::Wikidata,
    name: "Advantage spreadsheet",
    extensions: &["adv"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x04, 0xF4, 0x04, 0xF4, 0x04, 0xF4, 0x04, 0xF4, 0x04, 0xF4, 0x04, 0xF4, 0x04,
                    0xF4, 0x04, 0xF4, 0x04, 0xF4, 0x04, 0xF4, 0x00, 0x04, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
