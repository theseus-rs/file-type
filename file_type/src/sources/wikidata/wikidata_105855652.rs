use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855652: FileFormat = FileFormat {
    id: 105_855_652,
    puid: "wikidata/105855652",
    name: "Olitext document",
    extensions: &["otx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4F, 0x4C, 0x49, 0x54, 0x45, 0x58, 0x54, 0x0A, 0x0A, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
