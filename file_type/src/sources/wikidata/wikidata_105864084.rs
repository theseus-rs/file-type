use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864084: FileFormat = FileFormat {
    id: 105_864_084,
    source_type: SourceType::Wikidata,
    name: "MathMagic equation File",
    extensions: &["mmf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5C, 0x49, 0x4E, 0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54, 0x49, 0x4F, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
