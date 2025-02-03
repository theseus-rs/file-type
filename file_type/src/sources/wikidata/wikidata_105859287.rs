use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859287: FileFormat = FileFormat {
    id: 105_859_287,
    source_type: SourceType::Wikidata,
    name: "TI bitmap",
    extensions: &["92i"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x2A, 0x54, 0x49, 0x39, 0x32, 0x2A, 0x2A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
