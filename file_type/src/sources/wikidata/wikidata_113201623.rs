use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113201623: FileFormat = FileFormat {
    id: 113_201_623,
    source_type: SourceType::Wikidata,
    name: "LiveMotion Project File, version 1",
    extensions: &["liv"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xF8, 0xD2, 0xBA, 0x10]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x47, 0x5A, 0x20, 0x31, 0x2E]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
