use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858473: FileFormat = FileFormat {
    id: 105_858_473,
    source_type: SourceType::Wikidata,
    name: "EyesWeb patch",
    extensions: &["eyw"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x02, 0x00, 0x0A, 0x00, 0x43, 0x50, 0x61,
                    0x74, 0x63, 0x68, 0x56, 0x69, 0x65, 0x77,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
