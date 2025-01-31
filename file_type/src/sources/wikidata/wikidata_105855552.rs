use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855552: FileFormat = FileFormat {
    id: 105_855_552,
    puid: "wikidata/105855552",
    name: "OyezForms saved rigid form (v9, v10)",
    extensions: &["olf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x07, 0x24, 0x4F, 0x4C, 0x46, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
