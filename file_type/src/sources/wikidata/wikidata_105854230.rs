use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854230: FileFormat = FileFormat {
    id: 105_854_230,
    puid: "wikidata/105854230",
    name: "AlpineQuest Map",
    extensions: &["aqm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x4C, 0x41, 0x54, 0x50, 0x41, 0x43, 0x4B, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
