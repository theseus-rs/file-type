use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856956: FileFormat = FileFormat {
    id: 105_856_956,
    source_type: SourceType::Wikidata,
    name: "Mind Games - Othello saved game",
    extensions: &["gam"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x74, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
