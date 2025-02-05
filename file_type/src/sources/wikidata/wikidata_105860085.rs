use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860085: FileFormat = FileFormat {
    id: 105_860_085,
    source_type: SourceType::Wikidata,
    name: "Delphine CIN video",
    extensions: &["cin"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0xAA, 0x55])],
            },
        }],
    }],
    related_formats: &[],
};
