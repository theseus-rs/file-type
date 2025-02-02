use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858463: FileFormat = FileFormat {
    id: 105_858_463,
    source_type: SourceType::Wikidata,
    name: "E-Mail message (Var. 2)",
    extensions: &["eml"],
    media_types: &["message/rfc822"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x72, 0x6F, 0x6D])],
            },
        }],
    }],
    related_formats: &[],
};
