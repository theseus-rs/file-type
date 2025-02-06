use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858415: FileFormat = FileFormat {
    id: 105_858_415,
    source_type: SourceType::Wikidata,
    name: "EasyLanguage Document",
    extensions: &["eld"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x4D, 0x57, 0x52, 0x4B, 0x41, 0x72, 0x65, 0x61, 0x2E, 0x77, 0x72, 0x6B,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
