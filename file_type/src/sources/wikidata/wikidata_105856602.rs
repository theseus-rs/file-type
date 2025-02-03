use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856602: FileFormat = FileFormat {
    id: 105_856_602,
    source_type: SourceType::Wikidata,
    name: "MUST music / song",
    extensions: &["wvz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x56, 0x5A, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
