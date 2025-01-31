use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865614: FileFormat = FileFormat {
    id: 105_865_614,
    puid: "wikidata/105865614",
    name: "ZSoft Palette",
    extensions: &["pal"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0B, 0x00, 0x5A, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
