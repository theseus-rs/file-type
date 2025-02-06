use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858339: FileFormat = FileFormat {
    id: 105_858_339,
    source_type: SourceType::Wikidata,
    name: "WinPharoah Ethernet traffic capture",
    extensions: &["eth"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1A, 0x35, 0x01, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
