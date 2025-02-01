use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856402: FileFormat = FileFormat {
    id: 105_856_402,
    puid: "wikidata/105856402",
    name: "3D WohnungsPlan project",
    extensions: &["wds"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x33, 0x44, 0x2D, 0x57, 0x6F, 0x68, 0x6E, 0x75, 0x6E, 0x67, 0x73, 0x50, 0x6C,
                    0x61, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
