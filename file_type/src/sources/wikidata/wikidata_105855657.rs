use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855657: FileFormat = FileFormat {
    id: 105_855_657,
    puid: "wikidata/105855657",
    name: "OOMMF Vector Field 1.0 format",
    extensions: &["ovf"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x20, 0x4F, 0x4F, 0x4D, 0x4D, 0x46, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
