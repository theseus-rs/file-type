use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857149: FileFormat = FileFormat {
    id: 105_857_149,
    source_type: SourceType::Wikidata,
    name: "Heavy Iron Package game archive data",
    extensions: &["hip", "hop"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x49, 0x50, 0x41, 0x00, 0x00, 0x00, 0x00, 0x50, 0x41, 0x43, 0x4B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
