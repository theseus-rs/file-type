use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858651: FileFormat = FileFormat {
    id: 105_858_651,
    source_type: SourceType::Wikidata,
    name: "Gabriel Knight 3 barn game data",
    extensions: &["brn"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x4B, 0x33, 0x21, 0x42, 0x61, 0x72, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
