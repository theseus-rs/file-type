use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864861: FileFormat = FileFormat {
    id: 105_864_861,
    puid: "wikidata/105864861",
    name: "Bloodrayne game data archive",
    extensions: &["pod"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x4F, 0x44, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
