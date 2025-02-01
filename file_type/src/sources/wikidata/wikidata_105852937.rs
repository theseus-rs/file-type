use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852937: FileFormat = FileFormat {
    id: 105_852_937,
    puid: "wikidata/105852937",
    name: "ID Software Sprite format",
    extensions: &["spr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x44, 0x53, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
