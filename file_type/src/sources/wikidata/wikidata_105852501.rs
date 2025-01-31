use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852501: FileFormat = FileFormat {
    id: 105_852_501,
    puid: "wikidata/105852501",
    name: "Cosmigo Pro Motion SPRites sequence/animation",
    extensions: &["spr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
