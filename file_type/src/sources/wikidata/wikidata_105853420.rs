use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853420: FileFormat = FileFormat {
    id: 105_853_420,
    puid: "wikidata/105853420",
    name: "SMIRT file",
    extensions: &["swm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x45, 0x47, 0x49, 0x4E, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
