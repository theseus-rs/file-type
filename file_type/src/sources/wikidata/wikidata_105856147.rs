use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856147: FileFormat = FileFormat {
    id: 105_856_147,
    puid: "wikidata/105856147",
    name: "NetWare Disk Heap",
    extensions: &["dhp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x69, 0x73, 0x6B, 0x20, 0x48, 0x65, 0x61, 0x70, 0x20, 0x20, 0x20, 0x20,
                    0x20, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
