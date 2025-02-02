use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861033: FileFormat = FileFormat {
    id: 105_861_033,
    source_type: SourceType::Wikidata,
    name: "Lexar Encrypted file",
    extensions: &["lrs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x45, 0x58, 0x41, 0x52, 0x45, 0x4E, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
