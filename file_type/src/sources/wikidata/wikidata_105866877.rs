use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866877: FileFormat = FileFormat {
    id: 105_866_877,
    source_type: SourceType::Wikidata,
    name: "Git pack format (v1)",
    extensions: &["pack"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x41, 0x43, 0x4B, 0x00, 0x00, 0x00, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
