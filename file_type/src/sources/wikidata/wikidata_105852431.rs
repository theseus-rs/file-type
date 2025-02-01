use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852431: FileFormat = FileFormat {
    id: 105_852_431,
    puid: "wikidata/105852431",
    name: "NeoBook for DOS Settings",
    extensions: &["set"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x65, 0x6F, 0x42, 0x6F, 0x6F, 0x6B, 0x20, 0x53, 0x65, 0x74, 0x75, 0x70,
                    0x20, 0x46, 0x69, 0x6C, 0x65, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
