use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858339: FileFormat = FileFormat {
    id: 105_858_339,
    puid: "wikidata/105858339",
    name: "WinPharoah Ethernet traffic capture",
    extensions: &["eth"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
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
