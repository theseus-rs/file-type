use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858102: FileFormat = FileFormat {
    id: 105_858_102,
    puid: "wikidata/105858102",
    name: "Infinity Engine Talk Table (v1)",
    extensions: &["tlk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x4C, 0x4B, 0x20, 0x56, 0x31, 0x20, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
