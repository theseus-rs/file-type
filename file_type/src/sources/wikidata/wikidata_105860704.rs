use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860704: FileFormat = FileFormat {
    id: 105_860_704,
    source_type: SourceType::Wikidata,
    name: "Ray Dream Designer scene",
    extensions: &["rd4"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x33, 0x44, 0x43, 0x20, 0x7B, 0x0D])],
            },
        }],
    }],
    related_formats: &[],
};
