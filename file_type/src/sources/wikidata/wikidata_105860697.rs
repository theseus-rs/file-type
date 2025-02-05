use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860697: FileFormat = FileFormat {
    id: 105_860_697,
    source_type: SourceType::Wikidata,
    name: "Syzygy tablebase distance-to-zero",
    extensions: &["rtbz"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD7, 0x66, 0x0C, 0xA5])],
            },
        }],
    }],
    related_formats: &[],
};
