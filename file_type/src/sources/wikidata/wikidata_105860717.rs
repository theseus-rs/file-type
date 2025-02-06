use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860717: FileFormat = FileFormat {
    id: 105_860_717,
    source_type: SourceType::Wikidata,
    name: "Syzygy tablebase win/draw/loss",
    extensions: &["rtbw"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x71, 0xE8, 0x23, 0x5D])],
            },
        }],
    }],
    related_formats: &[],
};
