use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857958: FileFormat = FileFormat {
    id: 105_857_958,
    source_type: SourceType::Wikidata,
    name: "Android boot image",
    extensions: &["img"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4E, 0x44, 0x52, 0x4F, 0x49, 0x44, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
