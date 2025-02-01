use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856340: FileFormat = FileFormat {
    id: 105_856_340,
    puid: "wikidata/105856340",
    name: "DeskMate worksheet",
    extensions: &["wks"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x0E, 0x57, 0x4B, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
