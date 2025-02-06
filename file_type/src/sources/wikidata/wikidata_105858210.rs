use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858210: FileFormat = FileFormat {
    id: 105_858_210,
    source_type: SourceType::Wikidata,
    name: "E-Mail message (Var. 4)",
    extensions: &["eml"],
    media_types: &["message/rfc822"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x6F, 0x3A, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
