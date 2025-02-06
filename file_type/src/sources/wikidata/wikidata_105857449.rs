use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857449: FileFormat = FileFormat {
    id: 105_857_449,
    source_type: SourceType::Wikidata,
    name: "Aladdin 4D Drawing (v3.x)",
    extensions: &["4d"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x4C, 0x44, 0x4E, 0x40])],
            },
        }],
    }],
    related_formats: &[],
};
