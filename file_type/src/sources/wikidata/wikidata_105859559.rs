use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859559: FileFormat = FileFormat {
    id: 105_859_559,
    puid: "wikidata/105859559",
    name: "A-10 Tank Killer game data archive",
    extensions: &["vol"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x4F, 0x4C, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
