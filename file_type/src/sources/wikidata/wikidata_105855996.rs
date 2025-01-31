use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855996: FileFormat = FileFormat {
    id: 105_855_996,
    puid: "wikidata/105855996",
    name: "Radix game data",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x53, 0x52, 0x65, 0x73, 0x3A, 0x52, 0x61, 0x64, 0x69, 0x78,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
