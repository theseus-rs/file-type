use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861569: FileFormat = FileFormat {
    id: 105_861_569,
    puid: "wikidata/105861569",
    name: "Leggless Music Editor module",
    extensions: &["lme"],
    media_types: &["audio/x-mod"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x4D, 0x45, 0x00, 0x28, 0x63, 0x29, 0x31, 0x39, 0x39, 0x30, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
