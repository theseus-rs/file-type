use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852402: FileFormat = FileFormat {
    id: 105_852_402,
    puid: "wikidata/105852402",
    name: "STFax Script",
    extensions: &["script"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0B, 0x53, 0x54, 0x46, 0x61, 0x78, 0x53, 0x63, 0x72, 0x69, 0x70, 0x74, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
