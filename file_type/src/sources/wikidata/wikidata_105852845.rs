use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852845: FileFormat = FileFormat {
    id: 105_852_845,
    puid: "wikidata/105852845",
    name: "Roland music sequence (generic)",
    extensions: &["svq"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x56, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
