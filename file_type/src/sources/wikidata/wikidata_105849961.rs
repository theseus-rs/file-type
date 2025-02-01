use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849961: FileFormat = FileFormat {
    id: 105_849_961,
    puid: "wikidata/105849961",
    name: "ChiWriter document (v3.x or older)",
    extensions: &["chi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5C, 0x31, 0x63, 0x77])],
            },
        }],
    }],
    related_formats: &[],
};
