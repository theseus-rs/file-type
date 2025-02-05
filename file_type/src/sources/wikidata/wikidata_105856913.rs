use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856913: FileFormat = FileFormat {
    id: 105_856_913,
    source_type: SourceType::Wikidata,
    name: "Bars and Pipes Guitar Player Chord",
    extensions: &["gchone"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x4C, 0x55, 0x45, 0x52, 0x49, 0x42, 0x42, 0x4F, 0x4E, 0x47, 0x43, 0x48,
                    0x4F, 0x4E, 0x45, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
