use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855485: FileFormat = FileFormat {
    id: 105_855_485,
    puid: "wikidata/105855485",
    name: "FidoCAD Library",
    extensions: &["fcl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x46, 0x49, 0x44, 0x4F, 0x4C, 0x49, 0x42, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
