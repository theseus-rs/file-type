use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858579: FileFormat = FileFormat {
    id: 105_858_579,
    source_type: SourceType::Wikidata,
    name: "Multipaint image (MSX mode 2)",
    extensions: &["bin"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x05, 0x0F, 0x20, 0x00, 0x18, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
