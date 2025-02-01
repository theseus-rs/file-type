use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850329: FileFormat = FileFormat {
    id: 105_850_329,
    puid: "wikidata/105850329",
    name: "CoverFactory Project",
    extensions: &["cfp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x20, 0x88, 0x87, 0x89, 0x20, 0x92, 0x94, 0x89, 0x00, 0x00, 0x0E, 0xC0, 0x40,
                    0x07, 0xC1, 0xFA, 0xFA,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
