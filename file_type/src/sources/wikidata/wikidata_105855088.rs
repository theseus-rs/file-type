use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855088: FileFormat = FileFormat {
    id: 105_855_088,
    puid: "wikidata/105855088",
    name: "Amiga Money categories (v1)",
    extensions: &["amm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x4D, 0x31, 0x43, 0x41, 0x54, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
