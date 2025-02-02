use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861854: FileFormat = FileFormat {
    id: 105_861_854,
    source_type: SourceType::Wikidata,
    name: "Atari Digi-Mix module",
    extensions: &["mix"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x49, 0x58, 0x31, 0x4C, 0x65, 0x4F, 0x6E, 0x41, 0x72, 0x44, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
