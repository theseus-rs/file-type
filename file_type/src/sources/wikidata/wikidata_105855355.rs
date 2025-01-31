use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855355: FileFormat = FileFormat {
    id: 105_855_355,
    puid: "wikidata/105855355",
    name: "Family Tree Legends data",
    extensions: &["ftl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x54, 0x4C, 0x65, 0x67, 0x65, 0x6E, 0x64, 0x73, 0x20, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
