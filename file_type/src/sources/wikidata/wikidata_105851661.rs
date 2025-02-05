use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851661: FileFormat = FileFormat {
    id: 105_851_661,
    source_type: SourceType::Wikidata,
    name: "Crimson Fields level",
    extensions: &["src"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x6D, 0x69, 0x73, 0x73, 0x69, 0x6F, 0x6E, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
