use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858255: FileFormat = FileFormat {
    id: 105_858_255,
    source_type: SourceType::Wikidata,
    name: "E-Mail message (Var. 3)",
    extensions: &["eml"],
    media_types: &["message/rfc822"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
