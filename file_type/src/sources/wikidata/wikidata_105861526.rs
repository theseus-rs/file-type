use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861526: FileFormat = FileFormat {
    id: 105_861_526,
    source_type: SourceType::Wikidata,
    name: "CINEMA 4D Layout",
    extensions: &["l4d"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x36, 0x43, 0x34, 0x44, 0x4C, 0x41, 0x59, 0x36, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
